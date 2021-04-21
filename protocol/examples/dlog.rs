use std::time::Instant;
use protocol::prover::{rndoracle, prove_dlog, verify_dlog};
use backend::{scalar, genG1, genG2, smul1, smul2};

pub fn main() {
    println!("---------------");

    let elem_1 = genG1!();
    let elem_2 = genG2!();
    let phi = (elem_1, elem_2);
    let start = Instant::now();
    rndoracle(phi);
    println!("[+] Random oracle ({:.2?})", start.elapsed());

    let elem_1 = smul1!(scalar!(100), genG1!());
    let elem_2 = smul2!(scalar!(100), genG2!());
    let phi = (elem_1, elem_2);
    let witness = scalar!(100);
    let start = Instant::now();
    let proof = prove_dlog(phi, witness);
    println!("[+] dlog proof ({:.2?})", start.elapsed());

    let G = genG1!();
    let H = genG2!();
    let start = Instant::now();
    let verified = verify_dlog(&G, &H, phi, proof).unwrap();
    println!("[+] dlog verify ({:.2?})", start.elapsed());
    assert!(verified);
}
