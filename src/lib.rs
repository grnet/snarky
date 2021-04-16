//! # snarky
//!
//! The `snarky` crate implements ...

mod constraints;
mod error;
pub mod flow;
pub mod dlog;
pub use backend::{Scalar, G1Elem, G2Elem};
pub use polynomials::Univariate;
pub use constraints::QAP;
