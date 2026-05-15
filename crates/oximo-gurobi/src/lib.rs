#![forbid(unsafe_code)]

mod options;
mod translate;

pub use options::{GurobiOptions, GurobiPresolve};
pub use translate::solve;

use oximo_core::{Model, ModelKind};
use oximo_solver::{Solver, SolverError, SolverResult};

#[derive(Debug, Default, Clone, Copy)]
pub struct Gurobi;

impl Solver for Gurobi {
    type Options = GurobiOptions;

    fn name(&self) -> &str {
        "gurobi"
    }

    fn supports(&self, kind: ModelKind) -> bool {
        matches!(kind, ModelKind::LP | ModelKind::MILP)
    }

    fn solve(&mut self, model: &Model, opts: &GurobiOptions) -> Result<SolverResult, SolverError> {
        translate::solve(model, opts)
    }
}
