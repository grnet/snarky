use std::time::Instant;
use snarky::{
    scalar, zero, one, rndscalar, pow, G1_gen, G2_gen, contained_in_group, 
    add_1, add_2, G1_zero, G2_zero, mult_1, mult_2, pair, hashG1,
};

fn main() {

    // Scalar definition
    let zero = zero!();
    let one = one!();
    let seven = scalar!(7);
    let nine = scalar!(9);

    // Random scalar definition
    use rand::RngCore;                  // Must be present for rndscalar
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let r = rndscalar!(rng);
    println!("[+] rndscalar ({:.2?})", start.elapsed());

    // Raising scalars to power
    let base = scalar!(666);
    let exp = 999_usize;
    let start = Instant::now();
    pow!(base, exp);                                        // 666 ^ 999
    println!("[+] pow ({:.2?})", start.elapsed());

    // Generators
    let G = G1_gen!();                                      // G1 generator
    let H = G2_gen!();                                      // G2 generator

    // Zero group elements
    let zero_1 = G1_zero!();
    let zero_2 = G2_zero!();

    // Multiplication
    let start = Instant::now();
    let _7G = mult_1!(G, seven);                            // 7G
    println!("[+] mult_1 ({:.2?})", start.elapsed());
    let start = Instant::now();
    let _9H = mult_2!(H, nine);                             // 9H
    println!("[+] mult_2 ({:.2?})", start.elapsed());

    // Addition
    let start = Instant::now();
    let _8G = add_1!(G, _7G);                               // G + 7G
    println!("[+] add_1 ({:.2?})", start.elapsed());

    assert_eq!(add_1!(_8G, zero_1), _8G);
    assert_eq!(add_1!(zero_1, _8G), _8G);

    let start = Instant::now();
    let _10H = mult_2!(H, nine);                            // H + 9H
    println!("[+] add_2 ({:.2?})", start.elapsed());

    assert_eq!(add_2!(_10H, zero_2), _10H);
    assert_eq!(add_2!(zero_2, _10H), _10H);

    // Check inclusion group
    assert!(contained_in_group!(_8G));                      //  8G E G1
    assert!(contained_in_group!(_10H));                     // 10H E G2

    // Pairing
    let start = Instant::now();
    let res = pair!(_7G, _9H);                              // 7G * 9H
    println!("[+] pair ({:.2?})", start.elapsed());

    // hash-G1
    use sha2::Digest;
    use std::convert::TryInto;

    use std::iter::FromIterator;
    let bytes: Vec<u8> = (0..5).collect();
    let start = Instant::now();
    hashG1!(&bytes);
    println!("[+] Computed G1-hash ({:.2?})", start.elapsed());
}
