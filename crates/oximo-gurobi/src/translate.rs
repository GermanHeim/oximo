use std::time::Instant;

use grb::expr::LinExpr;
use grb::prelude::*;
use oximo_core::{ConstraintId, Domain, Model, ModelKind, ObjectiveSense, Sense, VarId};
use oximo_expr::{ExprArena, ExprId, LinearTerms, extract_linear};
use oximo_solver::{SolverError, SolverResult, SolverStatus};
use rustc_hash::FxHashMap;

use crate::GurobiOptions;
use crate::nonlinear::{LoweredExpr, LoweringCtx, lower};
use crate::options::apply as apply_options;

fn map_grb_err(e: grb::Error) -> SolverError {
    SolverError::Backend(e.to_string())
}

/// Translate `model` into a Gurobi model, solve, and return the generic
/// [`SolverResult`].
///
/// # Errors
///
/// Returns a [`SolverError`] if the model is unsupported, contains nonlinear
/// expressions Gurobi cannot represent, or if Gurobi reports an error during
/// setup or optimization.
///
/// # Panics
///
/// Panics if model variable or constraint indices overflow `u32`.
pub fn solve(model: &Model, opts: &GurobiOptions) -> Result<SolverResult, SolverError> {
    let kind = model.kind();
    let nonlinear_kind =
        matches!(kind, ModelKind::QP | ModelKind::MIQP | ModelKind::NLP | ModelKind::MINLP);

    let arena = model.arena();
    let vars = model.variables();
    let constraints = model.constraints();
    let objective = model.try_objective().map_err(SolverError::Core)?;

    let env = Env::new("").map_err(|e| SolverError::Backend(format!("Gurobi env: {e}")))?;
    let mut grb_model = grb::Model::with_env("oximo", &env).map_err(map_grb_err)?;

    let mut gurobi_vars = Vec::with_capacity(vars.len());
    for (i, v) in vars.iter().enumerate() {
        let vtype = match v.domain {
            Domain::Real => VarType::Continuous,
            Domain::Integer => VarType::Integer,
            Domain::Binary => VarType::Binary,
            Domain::SemiContinuous { .. } => VarType::SemiCont,
            Domain::SemiInteger { .. } => VarType::SemiInt,
        };
        // `add_var!` expands the f64 bounds with an `as f64` cast.
        #[allow(clippy::unnecessary_cast)]
        let gvar = add_var!(grb_model, vtype, bounds: v.lb..v.ub, name: &format!("x{i}"))
            .map_err(map_grb_err)?;
        gurobi_vars.push(gvar);
        if let Some(val) = v.initial {
            grb_model.set_obj_attr(attr::Start, &gvar, val).map_err(map_grb_err)?;
        }
    }

    let arena_ref: &ExprArena = &arena;

    // Linear fast path per constraint. We fall back to the general lowering only
    // for those that need it. Keep linear constraints tracked so duals/Pi can
    // still be reported in the LP/MILP case.
    let con_lin_terms: Vec<Option<LinearTerms>> =
        constraints.iter().map(|c| extract_linear(arena_ref, c.lhs)).collect();

    let mut gurobi_constrs: Vec<Option<grb::Constr>> = Vec::with_capacity(constraints.len());
    let mut aux_counter = 0_u32;

    for (c_id, (c, lin)) in constraints.iter().zip(con_lin_terms).enumerate() {
        if let Some(t) = lin {
            let adjusted_rhs = c.rhs - t.constant;
            let mut expr = LinExpr::new();
            for (v, co) in t.coeffs {
                expr.add_term(co, gurobi_vars[v.index()]);
            }
            let name = format!("c{c_id}");
            let constr = match c.sense {
                Sense::Le => {
                    grb_model.add_constr(&name, c!(expr <= adjusted_rhs)).map_err(map_grb_err)?
                }
                Sense::Ge => {
                    grb_model.add_constr(&name, c!(expr >= adjusted_rhs)).map_err(map_grb_err)?
                }
                Sense::Eq => {
                    grb_model.add_constr(&name, c!(expr == adjusted_rhs)).map_err(map_grb_err)?
                }
            };
            gurobi_constrs.push(Some(constr));
        } else {
            add_nonlinear_constraint(
                &arena,
                c.lhs,
                c.sense,
                c.rhs,
                c_id,
                &mut grb_model,
                &gurobi_vars,
                &mut aux_counter,
            )?;
            gurobi_constrs.push(None);
        }
    }

    let obj_constant = set_objective(
        &arena,
        objective.expr,
        objective.sense,
        &mut grb_model,
        &gurobi_vars,
        &mut aux_counter,
    )?;

    apply_options(&mut grb_model, opts).map_err(map_grb_err)?;
    if nonlinear_kind {
        // Gurobi requires NonConvex=2 for general nonlinear constraints and
        // bilinear non-convex objectives. Skip if the user already set it.
        let current = grb_model.get_param(grb::param::NonConvex).map_err(map_grb_err)?;
        if current < 2 {
            grb_model.set_param(grb::param::NonConvex, 2).map_err(map_grb_err)?;
        }
    }

    let started = Instant::now();
    grb_model.optimize().map_err(map_grb_err)?;
    let elapsed = started.elapsed();

    let status = map_status(&grb_model)?;
    let (primal, reduced_costs, dual) =
        collect_solution(&status, kind, &grb_model, &gurobi_vars, &gurobi_constrs);

    let objective_value = grb_model.get_attr(attr::ObjVal).ok().map(|v| v + obj_constant);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let iterations = grb_model.get_attr(attr::IterCount).unwrap_or(0.0) as u64;

    Ok(SolverResult {
        status,
        objective: objective_value,
        primal,
        dual,
        reduced_costs,
        solve_time: elapsed,
        iterations,
        raw_log: None,
    })
}

#[allow(clippy::too_many_arguments)]
fn add_nonlinear_constraint(
    arena: &ExprArena,
    lhs: ExprId,
    sense: Sense,
    rhs: f64,
    c_id: usize,
    grb_model: &mut grb::Model,
    gurobi_vars: &[grb::Var],
    aux_counter: &mut u32,
) -> Result<(), SolverError> {
    let mut ctx = LoweringCtx { model: grb_model, gurobi_vars, aux_counter: *aux_counter };
    let lowered = lower(arena, lhs, &mut ctx)?;
    *aux_counter = ctx.aux_counter;
    let name = format!("c{c_id}");
    match lowered {
        LoweredExpr::Linear(e) => {
            match sense {
                Sense::Le => grb_model.add_constr(&name, c!(e <= rhs)),
                Sense::Ge => grb_model.add_constr(&name, c!(e >= rhs)),
                Sense::Eq => grb_model.add_constr(&name, c!(e == rhs)),
            }
            .map_err(map_grb_err)?;
        }
        LoweredExpr::Quadratic(e) => {
            match sense {
                Sense::Le => grb_model.add_qconstr(&name, c!(e <= rhs)),
                Sense::Ge => grb_model.add_qconstr(&name, c!(e >= rhs)),
                Sense::Eq => grb_model.add_qconstr(&name, c!(e == rhs)),
            }
            .map_err(map_grb_err)?;
        }
        LoweredExpr::Var(v) => {
            match sense {
                Sense::Le => grb_model.add_constr(&name, c!(v <= rhs)),
                Sense::Ge => grb_model.add_constr(&name, c!(v >= rhs)),
                Sense::Eq => grb_model.add_constr(&name, c!(v == rhs)),
            }
            .map_err(map_grb_err)?;
        }
    }
    Ok(())
}

fn set_objective(
    arena: &ExprArena,
    obj_expr: ExprId,
    sense: ObjectiveSense,
    grb_model: &mut grb::Model,
    gurobi_vars: &[grb::Var],
    aux_counter: &mut u32,
) -> Result<f64, SolverError> {
    let grb_sense = match sense {
        ObjectiveSense::Minimize => ModelSense::Minimize,
        ObjectiveSense::Maximize => ModelSense::Maximize,
    };
    if let Some(t) = extract_linear(arena, obj_expr) {
        let mut e = LinExpr::new();
        for (v, c) in t.coeffs {
            e.add_term(c, gurobi_vars[v.index()]);
        }
        // Gurobi's set_objective absorbs LinExpr offsets into ObjCon, so we do
        // not need to track the constant separately.
        e.add_constant(t.constant);
        grb_model.set_objective(e, grb_sense).map_err(map_grb_err)?;
        return Ok(0.0);
    }
    let mut ctx = LoweringCtx { model: grb_model, gurobi_vars, aux_counter: *aux_counter };
    let lowered = lower(arena, obj_expr, &mut ctx)?;
    *aux_counter = ctx.aux_counter;
    grb_model.set_objective(lowered.into_expr_for_objective(), grb_sense).map_err(map_grb_err)?;
    Ok(0.0)
}

fn collect_solution(
    status: &SolverStatus,
    kind: ModelKind,
    model: &grb::Model,
    vars: &[grb::Var],
    constrs: &[Option<grb::Constr>],
) -> (FxHashMap<VarId, f64>, FxHashMap<VarId, f64>, FxHashMap<ConstraintId, f64>) {
    // `has_solution` only flags Optimal/Feasible, but Gurobi often holds an
    // incumbent on TimeLimit/IterationLimit/NodeLimit too.
    let sol_count = model.get_attr(attr::SolCount).unwrap_or(0);
    if sol_count <= 0 && !status.has_solution() {
        return (FxHashMap::default(), FxHashMap::default(), FxHashMap::default());
    }

    let primal_vals = model.get_obj_attr_batch(attr::X, vars.iter().copied()).ok();
    let primal = primal_vals
        .map(|v| {
            v.into_iter()
                .enumerate()
                .map(|(i, val)| (VarId(u32::try_from(i).unwrap()), val))
                .collect()
        })
        .unwrap_or_default();

    // Skip retrieval of duals and reduced costs for any model
    // class where Gurobi will either return zeros or refuse the attribute.
    if !matches!(kind, ModelKind::LP) {
        return (primal, FxHashMap::default(), FxHashMap::default());
    }

    let rc_vals = model.get_obj_attr_batch(attr::RC, vars.iter().copied()).ok();
    let reduced_costs = rc_vals
        .map(|v| {
            v.into_iter()
                .enumerate()
                .map(|(i, val)| (VarId(u32::try_from(i).unwrap()), val))
                .collect()
        })
        .unwrap_or_default();

    let mut dual = FxHashMap::default();
    for (i, c) in constrs.iter().enumerate() {
        if let Some(c) = c {
            if let Ok(pi) = model.get_obj_attr(attr::Pi, c) {
                dual.insert(ConstraintId(u32::try_from(i).unwrap()), pi);
            }
        }
    }

    (primal, reduced_costs, dual)
}

fn map_status(model: &grb::Model) -> Result<SolverStatus, SolverError> {
    let status = model.get_attr(attr::Status).map_err(map_grb_err)?;
    Ok(match status {
        Status::Optimal => SolverStatus::Optimal,
        Status::Infeasible => SolverStatus::Infeasible,
        Status::Unbounded | Status::InfOrUnbd => SolverStatus::Unbounded,
        Status::Numeric => SolverStatus::NumericError,
        Status::TimeLimit | Status::IterationLimit | Status::NodeLimit => SolverStatus::TimeLimit,
        Status::SubOptimal => SolverStatus::Feasible,
        Status::Loaded => SolverStatus::NotSolved,
        _ => SolverStatus::Other(format!("Status: {status:?}")),
    })
}
