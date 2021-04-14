//! # snarky
//!
//! The `snarky` crate implements ...

mod polynomial;
mod constraints;
mod util;
mod error;
pub mod flow;
pub mod dlog;
pub use backend::{Scalar, G1Elem, G2Elem};
pub use polynomial::Univariate;
pub use constraints::QAP;
