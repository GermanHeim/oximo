//! GAMS writer and backend for oximo.
//!
//! Translates an oximo [`Model`] into a GAMS `.gms` file, invokes the GAMS
//! executable via [`std::process::Command`], and parses the solution from a
//! PUT-generated text file.
//!
//! # Requirements
//!
//! A licensed GAMS installation must be available. The executable is resolved
//! from `PATH` by default, but can be overridden with [`GamsOptions::gams_path`] or
//! [`Gams::with_exec`].
//!
//! # Supported options
//!
//! Common option ([`CommonOptions`](oximo_solver::CommonOptions)) honored:
//!
//! | Field        | GAMS statement                                                    |
//! |--------------|-------------------------------------------------------------------|
//! | `time_limit` | `option ResLim = <seconds>;`                                      |
//! | `mip_gap`    | `option OptCR = <gap>;`                                           |
//! | `threads`    | `option threads = <n>;`                                           |
//! | `verbose`    | Forwards GAMS stdout/stderr to `raw_log` (suppresses `lo=0` flag) |
//!
//! GAMS-specific options ([`GamsOptions`]):
//!
//! | Field       | Description                                                                   |
//! |-------------|-------------------------------------------------------------------------------|
//! | `solver`    | Sub-solver name ([`GamsSolver::Baron`], `Cplex`, `Custom("MOSEK".into())`, …) |
//! | `gams_path` | Path to the `gams` executable                                                 |
//!
//! # Per-solver typed options
//!
//! Pass a [`GamsSolverConfig`] to [`GamsOptions::solver`] to select a sub-solver
//! and write a typed `<solver>.opt` file. GAMS picks it up via `model.optfile = 1`.
#![forbid(unsafe_code)]

mod options;
mod solver_options;
mod translate;

pub use options::{GamsOptions, GamsSolver};
pub use solver_options::{
    GamsBaronOptions, GamsCbcCuts, GamsCbcOptions, GamsCbcPresolve, GamsCplexMipEmphasis,
    GamsCplexOptions, GamsGurobiMipFocus, GamsGurobiOptions, GamsHighsOptions, GamsHighsPresolve,
    GamsHighsSolver, GamsIpoptLinearSolver, GamsIpoptMuStrategy, GamsIpoptOptions,
    GamsKnitroAlgorithm, GamsKnitroOptions, GamsMosekOptions, GamsScipOptions, GamsSolverConfig,
    GamsXpressOptions,
};
pub use translate::solve;

use oximo_core::{Model, ModelKind};
use oximo_solver::{Solver, SolverError, SolverResult};

/// GAMS solver backend.
///
/// Writes the model to a temporary `.gms` file, invokes the GAMS executable,
/// and returns the parsed [`SolverResult`].
#[derive(Debug, Default, Clone)]
pub struct Gams {
    /// Optional override for the GAMS executable path. When `None`, `"gams"` is
    /// looked up from the system `PATH`. Overridden per-call by
    /// [`GamsOptions::gams_path`].
    pub exec: Option<String>,
}

impl Gams {
    /// Create a backend that uses `gams` from `PATH`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a backend pointing at an explicit GAMS executable path.
    pub fn with_exec(path: impl Into<String>) -> Self {
        Self { exec: Some(path.into()) }
    }
}

impl Solver for Gams {
    type Options = GamsOptions;

    fn name(&self) -> &str {
        "gams"
    }

    fn supports(&self, kind: ModelKind) -> bool {
        matches!(kind, ModelKind::LP | ModelKind::MILP)
    }

    fn solve(&mut self, model: &Model, opts: &GamsOptions) -> Result<SolverResult, SolverError> {
        translate::solve(model, opts, self.exec.as_deref())
    }
}
