use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::prover::{Dlog, ProofError};
use util::map;

use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_std::rand::RngCore;
use ark_bls12_381;

#[test]
fn test_dlog_proof() {
    let parametrization = map! {
        (100_u64, 100_u64, 100_u64) => true,
        (666_u64, 100_u64, 100_u64) => false,
        (100_u64, 666_u64, 100_u64) => false,
        (100_u64, 100_u64, 666_u64) => false
    };
    for ((f1, f2, w), expected) in parametrization {
        let ctx = (&genG1!(), &genG2!());
        let elm1 = smul1!(scalar!(f1), genG1!());
        let elm2 = smul2!(scalar!(f2), genG2!());
        let commit = (elm1, elm2);
        let witness = scalar!(w);
        let proof = Dlog::prove(&commit, witness);
        match expected {
            true => {
                assert!(Dlog::verify(ctx, &commit, &proof).unwrap());
            },
            false => {
                assert_eq!(
                    Dlog::verify(ctx, &commit, &proof).unwrap_err(), 
                    ProofError::DlogFailure
                );
            }
        };
    }
}
