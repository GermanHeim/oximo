//! Live Gurobi tests for QP, NLP, and MINLP models.

use oximo_core::prelude::*;
use oximo_gurobi::{Gurobi, GurobiOptions};
use oximo_solver::{Solver, SolverStatus};

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn qp_min_sum_of_squares() {
    // min x^2 + y^2 s.t. x + y >= 1.
    // Optimum at x = y = 0.5, objective = 0.5.
    let m = Model::new("qp");
    let x = m.var("x").lb(-10.0).ub(10.0).build();
    let y = m.var("y").lb(-10.0).ub(10.0).build();
    m.constraint("c", (x + y).ge(1.0));
    m.minimize(x.powi(2) + y.powi(2));

    let r = Gurobi.solve(&m, &GurobiOptions::default()).expect("solve");
    assert!(matches!(r.status, SolverStatus::Optimal | SolverStatus::Feasible));
    let obj = r.objective.expect("obj");
    assert!(close(obj, 0.5, 1e-4), "obj = {obj}");
}

#[test]
fn nlp_with_sin_objective() {
    // min (x - 1)^2 + 0.1 * sin(x)^2 over x in [-3, 3].
    // Local minimum near x = 1, objective near 0.
    let m = Model::new("nlp_sin");
    let x = m.var("x").lb(-3.0).ub(3.0).initial(0.5).build();
    let one = Expr::constant(x.arena, 1.0);
    m.minimize((x - one).powi(2) + Expr::constant(x.arena, 0.1) * x.sin().powi(2));

    let r = Gurobi.solve(&m, &GurobiOptions::default()).expect("solve");
    assert!(matches!(r.status, SolverStatus::Optimal | SolverStatus::Feasible));
    let primal_x = r.primal.get(&VarId(0)).copied().expect("primal");
    assert!(close(primal_x, 1.0, 0.1), "x = {primal_x}");
}

#[test]
fn minlp_binary_with_log() {
    // Binary b, continuous x in [0.1, 10]. Min (x - 1)^2 + b * log(1 + x).
    // Optimal: b = 0, x = 1, objective = 0.
    let m = Model::new("minlp_log");
    let b = m.var("b").binary().build();
    let x = m.var("x").lb(0.1).ub(10.0).initial(0.5).build();
    let one = Expr::constant(x.arena, 1.0);
    m.minimize((x - one).powi(2) + b * (one + x).log());

    let r = Gurobi.solve(&m, &GurobiOptions::default()).expect("solve");
    assert!(matches!(r.status, SolverStatus::Optimal | SolverStatus::Feasible));
    let obj = r.objective.expect("obj");
    assert!(close(obj, 0.0, 1e-3), "obj = {obj}");
}
