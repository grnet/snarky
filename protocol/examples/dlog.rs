use std::time::Instant;
use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::prover::Dlog;
use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_std::rand::RngCore;
use ark_bls12_381;

pub fn main() {
    println!("---------------");

    let elm1 = genG1!();
    let elm2 = genG2!();
    let commit = (elm1, elm2);
    let start = Instant::now();
    Dlog::rndoracle(&commit);
    println!("[+] Random oracle ({:.2?})", start.elapsed());

    let elm1 = smul1!(100_u64, genG1!());
    let elm2 = smul2!(scalar!(100_u64), genG2!());
    let commit = (elm1, elm2);
    let witness = scalar!(100_u64);
    let start = Instant::now();
    let proof = Dlog::prove(&commit, witness);
    println!("[+] dlog proof ({:.2?})", start.elapsed());

    let ctx = (&genG1!(), &genG2!());
    let start = Instant::now();
    let verified = Dlog::verify(ctx, &commit, &proof).unwrap();
    println!("[+] dlog verify ({:.2?})", start.elapsed());
    assert!(verified);
}
