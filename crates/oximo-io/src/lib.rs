//! Model serialization for oximo: MPS and LP writers.
//!
//! TODO: NL format for nonlinear models.
#![forbid(unsafe_code)]

pub mod error;
pub mod lp;
pub mod mps;

pub use error::IoError;
pub use lp::{to_lp_string, write_lp};
pub use mps::{to_mps_string, write_mps};
