//! Arena-allocated expression tree for oximo.
//!
//! Expressions are stored as nodes in an [`ExprArena`]. User code holds
//! lightweight [`Expr`] handles that combine an [`ExprId`] with a borrow of
//! the owning arena. The arena keeps construction cache-friendly, makes deep
//! cloning of subtrees free (just copy IDs), and provides a stable substrate
//! for evaluation and simplification.

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
