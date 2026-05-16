use std::fmt::Write as FmtWrite;
use std::path::PathBuf;

use oximo_solver::{HasUniversal, UniversalOptions};

use crate::solver_options::GamsSolverConfig;

/// GAMS-specific solver options.
#[derive(Clone, Debug, Default)]
pub struct GamsOptions {
    pub universal: UniversalOptions,
    pub mip_gap: Option<f64>,
    /// Sub-solver selection with optional typed options.
    /// Translates to `option {LP|MIP} = <name>;` plus a `<solver>.opt` file
    /// when options are set.
    pub solver: Option<GamsSolverConfig>,
    /// Override for the `gams` executable. When `None`, `"gams"` is looked up
    /// from `PATH`.
    pub gams_path: Option<PathBuf>,
}

/// Named GAMS sub-solver. Use [`GamsSolver::Custom`] for any name that isn't
/// a pre-enumerated variant.
///
/// Reference: https://www.gams.com/latest/docs/S_MAIN.html#SOLVERS_MODEL_TYPES
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GamsSolver {
    /// ALPHAECP: MINLP
    AlphaEcp,
    /// ANTIGONE: LP, MIP, NLP, MINLP, QCP, MIQCP, Global
    Antigone,
    /// BARON: LP, MIP, NLP, MCP, MPEC, CNS, MINLP, QCP, MIQCP, Global
    Baron,
    /// CBC: LP, MIP
    Cbc,
    /// CONOPT: NLP, DNLP, CNS, MPEC
    Conopt,
    /// COPT: LP, MIP, QCP, MIQCP
    Copt,
    /// CPLEX: LP, MIP, QCP, MIQCP
    Cplex,
    /// DECIS: LP, Stochastic
    Decis,
    /// DICOPT: MINLP
    Dicopt,
    /// GLPK: LP, MIP (not in GAMS docs but recognized)
    Glpk,
    /// GUROBI: LP, MIP, NLP, MINLP, QCP, MIQCP, Global
    Gurobi,
    /// GUSS: LP, MIP, NLP, MCP, MPEC, CNS, DNLP, MINLP, QCP, MIQCP
    Guss,
    /// HiGHS: LP, MIP
    Highs,
    /// IPOPT: NLP, DNLP, CNS, MPEC
    Ipopt,
    /// JAMS: MPEC
    Jams,
    /// KESTREL: all model types (remote solver submission)
    Kestrel,
    /// KNITRO: LP, MIP, NLP, MCP, MPEC, CNS, DNLP, MINLP, QCP, MIQCP
    Knitro,
    /// LINDO: LP, MIP, NLP, MCP, MPEC, CNS, DNLP, MINLP, QCP, Stochastic, Global
    Lindo,
    /// LINDOGLOBAL: LP, MIP, NLP, MINLP, QCP, MIQCP, Global
    LindoGlobal,
    /// MILES: MCP
    Miles,
    /// MINOS: NLP, DNLP, CNS, MPEC
    Minos,
    /// MOSEK: LP, MIP, NLP, QCP, MIQCP
    Mosek,
    /// NLPEC: NLP, MPEC
    Nlpec,
    /// ODHCPLEX: LP, MIP
    OdhCplex,
    /// PATH: MCP, MPEC
    Path,
    /// QUADMINOS: NLP
    QuadMinos,
    /// RESHOP: NLP
    Reshop,
    /// SBB: NLP, MINLP
    Sbb,
    /// SCIP: LP, MIP, NLP, MINLP, QCP, MIQCP, Global
    Scip,
    /// SHOT: MINLP
    Shot,
    /// SNOPT: NLP, DNLP, CNS, MPEC
    Snopt,
    /// SOPLEX: LP
    Soplex,
    /// XPRESS: LP, MIP, NLP, MINLP, QCP, MIQCP, Global
    Xpress,
    /// Any other GAMS-recognized solver name, emitted verbatim.
    Custom(String),
}

impl GamsSolver {
    /// GAMS solver keyword used in the `option {LP|MIP} = ...;` statement.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::AlphaEcp => "ALPHAECP",
            Self::Antigone => "ANTIGONE",
            Self::Baron => "BARON",
            Self::Cbc => "CBC",
            Self::Conopt => "CONOPT",
            Self::Copt => "COPT",
            Self::Cplex => "CPLEX",
            Self::Decis => "DECIS",
            Self::Dicopt => "DICOPT",
            Self::Glpk => "GLPK",
            Self::Gurobi => "GUROBI",
            Self::Guss => "GUSS",
            Self::Highs => "HIGHS",
            Self::Ipopt => "IPOPT",
            Self::Jams => "JAMS",
            Self::Kestrel => "KESTREL",
            Self::Knitro => "KNITRO",
            Self::Lindo => "LINDO",
            Self::LindoGlobal => "LINDOGLOBAL",
            Self::Miles => "MILES",
            Self::Minos => "MINOS",
            Self::Mosek => "MOSEK",
            Self::Nlpec => "NLPEC",
            Self::OdhCplex => "ODHCPLEX",
            Self::Path => "PATH",
            Self::QuadMinos => "QUADMINOS",
            Self::Reshop => "RESHOP",
            Self::Sbb => "SBB",
            Self::Scip => "SCIP",
            Self::Shot => "SHOT",
            Self::Snopt => "SNOPT",
            Self::Soplex => "SOPLEX",
            Self::Xpress => "XPRESS",
            Self::Custom(s) => s.as_str(),
        }
    }
}

impl GamsOptions {
    #[must_use]
    pub fn mip_gap(mut self, gap: f64) -> Self {
        self.mip_gap = Some(gap);
        self
    }

    #[must_use]
    pub fn solver(mut self, s: impl Into<GamsSolverConfig>) -> Self {
        self.solver = Some(s.into());
        self
    }

    #[must_use]
    pub fn gams_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.gams_path = Some(p.into());
        self
    }
}

impl HasUniversal for GamsOptions {
    fn universal(&self) -> &UniversalOptions {
        &self.universal
    }

    fn universal_mut(&mut self) -> &mut UniversalOptions {
        &mut self.universal
    }
}

/// Emit GAMS option statements into `gms` before the `Solve` statement.
///
/// `solve_type` is `"LP"` or `"MIP"`, used to scope the `solver` option.
pub fn write_options(gms: &mut String, o: &GamsOptions, solve_type: &str) {
    if let Some(d) = o.universal.time_limit {
        writeln!(gms, "option ResLim = {};", d.as_secs_f64()).unwrap();
    }
    if let Some(g) = o.mip_gap {
        writeln!(gms, "option OptCR = {g};").unwrap();
    }
    if let Some(n) = o.universal.threads {
        writeln!(gms, "option threads = {n};").unwrap();
    }
    if let Some(s) = &o.solver {
        writeln!(gms, "option {solve_type} = {};", s.gams_name()).unwrap();
    }
}
