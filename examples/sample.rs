use std::time::Instant;
use snarky::flow::{QAP, Trapdoor, setup, update, verify};

use snarky::{
    scalar, zero, one, rand_scalar,
    G1_gen, G2_gen,
    mult_1, mult_2, pair,
};

use rand::RngCore;  // SOS: Must be present for rand_scalar


fn main() {

    // How to use backend macros
    
    // Define scalars
    let zero = zero!();
    let one = one!();
    let factor_1 = scalar!(7);
    let factor_2 = scalar!(9);

    // Generate random scalar
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let r = rand_scalar!(rng);

    // Pairing
    let G = G1_gen!();                  // G1 generator
    let H = G2_gen!();                  // G2 generator
    let left  = mult_1!(G, factor_1);   // 7G
    let right = mult_2!(H, factor_2);   // 9H
    let res = pair!(left, right);       // 7G * 9H

    // How to use polynomial
    
    use snarky::Univariate;
    let poly = Univariate::create(vec![
        scalar!(1), scalar!(2), scalar!(3),
    ]);
    let r = poly.evaluate(scalar!(7)).unwrap();
    println!("{}", r);
    println!("{}", poly.coeff(1));
    println!("{}", poly.degree());


    let start = Instant::now();
    println!("--------------------------");

    let l = 5;
    let m = 4;
    let n = 3;
    let qap = QAP::create_default(l, m, n);
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);

    println!("--------------------------");
    println!("Time elaped: {:.2?}", start.elapsed());
}
