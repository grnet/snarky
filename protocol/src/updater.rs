use crate::prover::{RhoProof, Witness, UpdateProof};
use crate::srs::{Trapdoor, SRS};
use circuits::ConstraintSystem;
use backend::*;
pub use crate::prover::BatchProof;

use ark_std::rand::Rng as ArkRng;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    ONE = 1,
    TWO = 2,
}

pub fn update(qap: &ConstraintSystem, srs: &mut SRS, batch: &mut BatchProof, phase: Phase) {

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


