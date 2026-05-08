//! Core modeling types for oximo: variables, parameters, sets, constraints,
//! objectives, and the [`Model`] container that owns the expression arena.
//!
//! See [`prelude`] for the canonical re-exports.

#![forbid(unsafe_code)]

pub mod constraint;
pub mod domain;
pub mod error;
pub mod indexed;
pub mod model;
pub mod objective;
pub mod param;
pub mod prelude;
pub mod set;
pub mod var;

pub use constraint::{Constraint, ConstraintExpr, ConstraintId, IntoRhs, Relate, Sense};
pub use domain::Domain;
pub use error::{Error, Result};
pub use indexed::IndexedVar;
pub use model::{IndexedVarBuilder, Model, ModelKind};
pub use objective::{Objective, ObjectiveSense};
pub use param::Parameter;
pub use set::{IndexKey, Set, SetIter};
pub use var::{VarBuilder, Variable};

// Re-export the expression handle so downstream code does not need a separate
// `oximo-expr` import.
pub use oximo_expr::{Expr, ExprArena, ExprId, ExprNode, ParamId, VarId};
