use grb::Model as GrbModel;
use grb::parameter::{DoubleParam, IntParam, StrParam};
use oximo_solver::{HasMip, HasUniversal, MipOptions, Presolve, UniversalOptions};

/// Gurobi-specific solver options.
///
/// Universal options (`time_limit`, `threads`, `verbose`) come from the embedded
/// [`UniversalOptions`] via [`UniversalOptionsExt`](oximo_solver::UniversalOptionsExt).
/// LP/MILP options (`mip_gap`, `presolve`) come from the embedded [`MipOptions`]
/// via [`MipOptionsExt`](oximo_solver::MipOptionsExt).
///
/// See the [Gurobi parameter reference](https://docs.gurobi.com/projects/optimizer/en/current/reference/parameters.html)
/// for semantics. Method names are the snake_case of the official parameter
/// name (e.g. `ConcurrentMIP` -> `.concurrent_mip(2)`).
#[derive(Clone, Debug, Default)]
pub struct GurobiOptions {
    pub universal: UniversalOptions,
    pub mip: MipOptions,
    int_params: Vec<(IntParam, i32)>,
    double_params: Vec<(DoubleParam, f64)>,
    str_params: Vec<(StrParam, String)>,
}

// Generates one typed builder method per Gurobi parameter.
macro_rules! gurobi_params {
    ($( ($kind:ident, $method:ident, $variant:ident) ),* $(,)?) => {
        $(gurobi_params!(@impl $kind, $method, $variant);)*
    };
    (@impl int, $method:ident, $variant:ident) => {
        #[must_use]
        pub fn $method(mut self, v: i32) -> Self {
            self.int_params.push((IntParam::$variant, v));
            self
        }
    };
    (@impl dbl, $method:ident, $variant:ident) => {
        #[must_use]
        pub fn $method(mut self, v: f64) -> Self {
            self.double_params.push((DoubleParam::$variant, v));
            self
        }
    };
    (@impl str, $method:ident, $variant:ident) => {
        #[must_use]
        pub fn $method(mut self, v: impl Into<String>) -> Self {
            self.str_params.push((StrParam::$variant, v.into()));
            self
        }
    };
}

impl GurobiOptions {
    gurobi_params!(
        // Termination
        (int, bar_iter_limit, BarIterLimit),
        (int, solution_limit, SolutionLimit),
        (dbl, cutoff, Cutoff),
        (dbl, iteration_limit, IterationLimit),
        (dbl, node_limit, NodeLimit),
        (dbl, best_obj_stop, BestObjStop),
        (dbl, best_bd_stop, BestBdStop),
        (dbl, work_limit, WorkLimit),
        // Tolerances
        (dbl, feasibility_tol, FeasibilityTol),
        (dbl, int_feas_tol, IntFeasTol),
        (dbl, markowitz_tol, MarkowitzTol),
        (dbl, mip_gap_abs, MIPGapAbs),
        (dbl, optimality_tol, OptimalityTol),
        (dbl, psd_tol, PSDTol),
        // Simplex
        (int, method, Method),
        (dbl, perturb_value, PerturbValue),
        (dbl, obj_scale, ObjScale),
        (int, scale_flag, ScaleFlag),
        (int, simplex_pricing, SimplexPricing),
        (int, quad, Quad),
        (int, norm_adjust, NormAdjust),
        (int, sifting, Sifting),
        (int, sift_method, SiftMethod),
        // Barrier
        (dbl, bar_conv_tol, BarConvTol),
        (int, bar_correctors, BarCorrectors),
        (int, bar_homogeneous, BarHomogeneous),
        (int, bar_order, BarOrder),
        (dbl, bar_qcp_conv_tol, BarQCPConvTol),
        (int, crossover, Crossover),
        (int, crossover_basis, CrossoverBasis),
        // MIP
        (int, branch_dir, BranchDir),
        (int, degen_moves, DegenMoves),
        (int, disconnected, Disconnected),
        (dbl, heuristics, Heuristics),
        (dbl, improve_start_gap, ImproveStartGap),
        (dbl, improve_start_time, ImproveStartTime),
        (dbl, improve_start_nodes, ImproveStartNodes),
        (int, integrality_focus, IntegralityFocus),
        (int, min_rel_nodes, MinRelNodes),
        (int, mip_focus, MIPFocus),
        (str, nodefile_dir, NodefileDir),
        (dbl, nodefile_start, NodefileStart),
        (int, node_method, NodeMethod),
        (dbl, no_rel_heur_time, NoRelHeurTime),
        (dbl, no_rel_heur_work, NoRelHeurWork),
        (int, pump_passes, PumpPasses),
        (int, rins, RINS),
        (str, sol_files, SolFiles),
        (int, start_node_limit, StartNodeLimit),
        (int, sub_mip_nodes, SubMIPNodes),
        (int, symmetry, Symmetry),
        (int, var_branch, VarBranch),
        (int, solution_number, SolutionNumber),
        (int, zero_obj_nodes, ZeroObjNodes),
        // Cuts
        (int, cuts, Cuts),
        (int, clique_cuts, CliqueCuts),
        (int, cover_cuts, CoverCuts),
        (int, flow_cover_cuts, FlowCoverCuts),
        (int, flow_path_cuts, FlowPathCuts),
        (int, gub_cover_cuts, GUBCoverCuts),
        (int, implied_cuts, ImpliedCuts),
        (int, proj_implied_cuts, ProjImpliedCuts),
        (int, mip_sep_cuts, MIPSepCuts),
        (int, mir_cuts, MIRCuts),
        (int, strong_cg_cuts, StrongCGCuts),
        (int, mod_k_cuts, ModKCuts),
        (int, zero_half_cuts, ZeroHalfCuts),
        (int, network_cuts, NetworkCuts),
        (int, sub_mip_cuts, SubMIPCuts),
        (int, inf_proof_cuts, InfProofCuts),
        (int, rlt_cuts, RLTCuts),
        (int, relax_lift_cuts, RelaxLiftCuts),
        (int, bqp_cuts, BQPCuts),
        (int, psd_cuts, PSDCuts),
        (int, lift_project_cuts, LiftProjectCuts),
        (int, mixing_cuts, MixingCuts),
        (int, cut_agg_passes, CutAggPasses),
        (int, cut_passes, CutPasses),
        (int, gomory_passes, GomoryPasses),
        // Distributed / Cloud / Token server
        (str, worker_pool, WorkerPool),
        (str, worker_password, WorkerPassword),
        (str, compute_server, ComputeServer),
        (str, token_server, TokenServer),
        (str, server_password, ServerPassword),
        (int, server_timeout, ServerTimeout),
        (str, cs_router, CSRouter),
        (str, cs_group, CSGroup),
        (dbl, cs_queue_timeout, CSQueueTimeout),
        (int, cs_priority, CSPriority),
        (int, cs_idle_timeout, CSIdleTimeout),
        (int, cs_tls_insecure, CSTLSInsecure),
        (int, ts_port, TSPort),
        (str, cloud_access_id, CloudAccessID),
        (str, cloud_secret_key, CloudSecretKey),
        (str, cloud_pool, CloudPool),
        (str, cloud_host, CloudHost),
        (str, cs_manager, CSManager),
        (str, cs_auth_token, CSAuthToken),
        (str, cs_api_access_id, CSAPIAccessID),
        (str, cs_api_secret, CSAPISecret),
        (int, cs_batch_mode, CSBatchMode),
        (str, username, Username),
        (str, cs_app_name, CSAppName),
        (int, cs_client_log, CSClientLog),
        // Tuning
        (dbl, tune_time_limit, TuneTimeLimit),
        (int, tune_results, TuneResults),
        (int, tune_criterion, TuneCriterion),
        (int, tune_trials, TuneTrials),
        (int, tune_output, TuneOutput),
        (int, tune_jobs, TuneJobs),
        (dbl, tune_cleanup, TuneCleanup),
        (int, tune_metric, TuneMetric),
        (dbl, tune_target_mip_gap, TuneTargetMIPGap),
        (dbl, tune_target_time, TuneTargetTime),
        (int, tune_dynamic_jobs, TuneDynamicJobs),
        // Multi-objective / scenarios / pool
        (int, obj_number, ObjNumber),
        (int, multi_obj_method, MultiObjMethod),
        (int, multi_obj_pre, MultiObjPre),
        (int, scenario_number, ScenarioNumber),
        (int, pool_solutions, PoolSolutions),
        (dbl, pool_gap, PoolGap),
        (dbl, pool_gap_abs, PoolGapAbs),
        (int, pool_search_mode, PoolSearchMode),
        // Presolve
        (int, pre_crush, PreCrush),
        (int, pre_dep_row, PreDepRow),
        (int, pre_dual, PreDual),
        (int, pre_passes, PrePasses),
        (int, pre_q_linearize, PreQLinearize),
        (dbl, pre_sos1_big_m, PreSOS1BigM),
        (dbl, pre_sos2_big_m, PreSOS2BigM),
        (int, pre_sparsify, PreSparsify),
        (int, pre_miqcp_form, PreMIQCPForm),
        (int, pre_sos1_encoding, PreSOS1Encoding),
        (int, pre_sos2_encoding, PreSOS2Encoding),
        (dbl, feas_relax_big_m, FeasRelaxBigM),
        // General
        (int, aggregate, Aggregate),
        (int, agg_fill, AggFill),
        (int, concurrent_mip, ConcurrentMIP),
        (int, concurrent_jobs, ConcurrentJobs),
        (int, concurrent_method, ConcurrentMethod),
        (int, display_interval, DisplayInterval),
        (int, distributed_mip_jobs, DistributedMIPJobs),
        (int, dual_reductions, DualReductions),
        (int, iis_method, IISMethod),
        (int, inf_unbd_info, InfUnbdInfo),
        (int, json_sol_detail, JSONSolDetail),
        (int, lazy_constraints, LazyConstraints),
        (str, log_file, LogFile),
        (int, log_to_console, LogToConsole),
        (int, miqcp_method, MIQCPMethod),
        (int, non_convex, NonConvex),
        (int, numeric_focus, NumericFocus),
        (int, qcp_dual, QCPDual),
        (int, record, Record),
        (str, result_file, ResultFile),
        (int, seed, Seed),
        (int, update_mode, UpdateMode),
        (int, ignore_names, IgnoreNames),
        (int, start_number, StartNumber),
        (int, partition_place, PartitionPlace),
        // Piecewise-linear / nonlinear
        (int, func_pieces, FuncPieces),
        (dbl, func_piece_length, FuncPieceLength),
        (dbl, func_piece_error, FuncPieceError),
        (dbl, func_piece_ratio, FuncPieceRatio),
        (dbl, func_max_val, FuncMaxVal),
        (int, func_nonlinear, FuncNonlinear),
        (int, nlp_heur, NLPHeur),
        // Memory / resources
        (dbl, mem_limit, MemLimit),
        (dbl, soft_mem_limit, SoftMemLimit),
        // Licensing / WLS
        (int, license_id, LicenseID),
        (str, user_name, UserName),
        (str, wls_access_id, WLSAccessID),
        (str, wls_secret, WLSSecret),
        (str, wls_token, WLSToken),
        (int, wls_token_duration, WLSTokenDuration),
        (dbl, wls_token_refresh, WLSTokenRefresh),
        // Misc
        (int, network_alg, NetworkAlg),
        (int, obbt, OBBT),
        (int, solution_target, SolutionTarget),
        (int, lp_warm_start, LPWarmStart),
        (str, job_id, JobID),
        (str, dummy, Dummy),
    );
}

impl HasUniversal for GurobiOptions {
    fn universal(&self) -> &UniversalOptions {
        &self.universal
    }

    fn universal_mut(&mut self) -> &mut UniversalOptions {
        &mut self.universal
    }
}

impl HasMip for GurobiOptions {
    fn mip(&self) -> &MipOptions {
        &self.mip
    }

    fn mip_mut(&mut self) -> &mut MipOptions {
        &mut self.mip
    }
}

/// Apply typed [`GurobiOptions`] onto a live Gurobi model.
pub(crate) fn apply(model: &mut GrbModel, o: &GurobiOptions) -> Result<(), grb::Error> {
    if let Some(d) = o.universal.time_limit {
        model.set_param(grb::param::TimeLimit, d.as_secs_f64())?;
    }
    if let Some(n) = o.universal.threads {
        model.set_param(grb::param::Threads, i32::try_from(n).unwrap_or(i32::MAX))?;
    }
    if let Some(b) = o.universal.verbose {
        model.set_param(grb::param::OutputFlag, i32::from(b))?;
    }
    if let Some(g) = o.mip.mip_gap {
        model.set_param(grb::param::MIPGap, g)?;
    }
    if let Some(p) = o.mip.presolve {
        let v = match p {
            Presolve::Off => 0,
            Presolve::Auto => -1,
            Presolve::On => 2,
        };
        model.set_param(grb::param::Presolve, v)?;
    }
    for (p, v) in &o.int_params {
        model.set_param(*p, *v)?;
    }
    for (p, v) in &o.double_params {
        model.set_param(*p, *v)?;
    }
    for (p, v) in &o.str_params {
        model.set_param(*p, v.clone())?;
    }
    Ok(())
}
