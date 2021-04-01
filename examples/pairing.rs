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
    let left  = mult_1!(G, factor_1);   // 7G
    let right = mult_2!(H, factor_2);   // 9H
    let res = pair!(left, right);       // 7G * 9H
}
