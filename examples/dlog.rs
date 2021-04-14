use std::time::Instant;
use snarky::dlog::{rndoracle, prove_dlog, verify_dlog};
use snarky::{scalar, G1_gen, G2_gen, mult_1, mult_2};

pub fn main() {
    println!("---------------");

    let elem_1 = G1_gen!();
    let elem_2 = G2_gen!();
    let phi = (elem_1, elem_2);
    let start = Instant::now();
    rndoracle(phi);
    println!("[+] Random oracle ({:.2?})", start.elapsed());

    let elem_1 = mult_1!(G1_gen!(), scalar!(100));
    let elem_2 = mult_2!(G2_gen!(), scalar!(100));
    let phi = (elem_1, elem_2);
    let witness = scalar!(100);
    let start = Instant::now();
    let proof = prove_dlog(phi, witness);
    println!("[+] dlog proof ({:.2?})", start.elapsed());

    let G = G1_gen!();
    let H = G2_gen!();
    let start = Instant::now();
    let verified = verify_dlog(&G, &H, phi, proof);
    println!("[+] dlog verify ({:.2?})", start.elapsed());
    assert!(verified);
}
