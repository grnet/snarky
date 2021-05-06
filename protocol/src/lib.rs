//! # snarky
//!
//! The `protocol` crate implements ...

pub mod srs;
pub mod prover;
pub mod updater;
pub mod verifier;

pub use srs::{Trapdoor, SRS};
pub use prover::BatchProof;
pub use updater::{Phase, update};
pub use verifier::{Verification, verify_naive};
