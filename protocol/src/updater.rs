use rand::RngCore;          // Must be in scope for update
use subtle::ConstantTimeEq; // Must be in scope for ct comparisons

use backend::*;
use circuits::QAP;
use crate::prover::{RhoProof, Witness, UpdateProof};

use backend::*;
use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_bls12_381;

use crate::srs::{Trapdoor, SRS};
pub use crate::prover::BatchProof;

use ark_std::rand::RngCore as ArkRngCore;
use ark_std::rand::SeedableRng;

use rayon::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    ONE = 1,
    TWO = 2,
}

pub fn update(qap: &QAP, srs: &mut SRS, batch: &mut BatchProof, phase: Phase) {

    // phase 1/2: step 2
    let witness = match phase {
        Phase::ONE => {
            Witness::ONE(
                rscalar!(::util::snarky_rng()),
                rscalar!(::util::snarky_rng()),
                rscalar!(::util::snarky_rng()),
            )
        },
        Phase::TWO => {
            Witness::TWO(rscalar!(
                ::util::snarky_rng()
            ))
        }
    };

    // phase 1: steps 3-7; phase 2: steps 3-4
    batch.append(UpdateProof::create(
        &srs, 
        &witness
    ));

    // phase 1: steps 8-10; phase 2: step 5
    srs.update(&qap, witness);
}


