#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod arena;
mod eval;
mod handle;
mod linear;
mod ops;
mod simplify;
mod visit;

pub use arena::{ExprArena, ExprId, ExprNode, ParamId, VarId};
pub use eval::{EvalContext, EvalError, evaluate};
pub use handle::Expr;
pub use linear::{LinearTerms, extract_linear};
pub use ops::sum;
pub use simplify::simplify;
pub use visit::{Visitor, walk};
