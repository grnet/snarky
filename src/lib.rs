//! # snarky
//!
//! The `snarky` crate implements ...

pub mod flow;
mod backend;
pub mod dlog;
pub use backend::{Scalar, Univariate, G1Elem, G2Elem};
mod util;
mod error;
