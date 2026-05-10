//! Model serialization for oximo.
//! For now, we only have an MPS writer
//!
//! TODO: Add LP and NL formats
#![forbid(unsafe_code)]

pub mod error;
pub mod mps;

pub use error::IoError;
pub use mps::{to_mps_string, write_mps};
