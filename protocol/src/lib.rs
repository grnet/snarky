//! # snarky
//!
//! The `protocol` crate implements ...

pub mod prover;
pub mod srs;
pub mod flow;

pub use flow::{
    SRS,
    Trapdoor,
    BatchProof,
    Phase,
    Verification,
    update,
    verify,
};
