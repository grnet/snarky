use std::time::Instant;
use snarky::{
    scalar, zero, one, rand_scalar, G1_gen, G2_gen,
    mult_1, mult_2, pair,
};

fn main() {

    // Define scalars
    let zero = zero!();
    let one = one!();
    let factor_1 = scalar!(7);
    let factor_2 = scalar!(9);

    // Generate random scalar
    use rand::RngCore;                  // Must be present for rand_scalar
    let mut rng = rand::thread_rng();
    let r = rand_scalar!(rng);

    // Pairing
    let G = G1_gen!();                  // G1 generator
    let H = G2_gen!();                  // G2 generator

    let start_1 = Instant::now();
    let left  = mult_1!(G, factor_1);   // 7G
    // let left  = G * factor_1;   // 7G
    println!("[+] mult_1 in ({:.2?})", start_1.elapsed());

    let start_2 = Instant::now();
    let right = mult_2!(H, factor_2);   // 9H
    println!("[+] mult_2 in ({:.2?})", start_1.elapsed());

    // let res = pair!(left, right);       // 7G * 9H
}
