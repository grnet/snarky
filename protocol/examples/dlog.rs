use std::time::Instant;
use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::prover::Dlog;
use ark_ec::AffineCurve;

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
