use std::time::Instant;
use backend::{
    scalar, zero, one, rscalar, pow, genG1, genG2, zeroG1, zeroG2,
    contained_in_group, add1, add2, smul1, smul2, pair, 
    bytes1, bytes2, hashG1,
};

fn main() {

    // Scalar definition
    let zero = zero!();
    let one = one!();
    let seven = scalar!(7);
    let nine = scalar!(9);

    // Random scalar definition
    use rand::RngCore;                  // Must be present for rscalar
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let r = rscalar!(rng);
    println!("[+] rscalar ({:.2?})", start.elapsed());

    // Raising scalars to power
    let base = scalar!(666);
    let exp = 999_usize;
    let start = Instant::now();
    pow!(base, exp);                                        // 666 ^ 999
    println!("[+] pow ({:.2?})", start.elapsed());

    // Generators
    let G = genG1!();                                      // G1 generator
    let H = genG2!();                                      // G2 generator

    // Zero group elements
    let zero_1 = zeroG1!();
    let zero_2 = zeroG2!();

    // Multiplication
    let start = Instant::now();
    let _7G = smul1!(G, seven);                            // 7G
    println!("[+] smul1 ({:.2?})", start.elapsed());
    let start = Instant::now();
    let _9H = smul2!(H, nine);                             // 9H
    println!("[+] smul2 ({:.2?})", start.elapsed());

    // Addition
    let start = Instant::now();
    let _8G = add1!(G, _7G);                               // G + 7G
    println!("[+] add1 ({:.2?})", start.elapsed());

    assert_eq!(add1!(_8G, zero_1), _8G);
    assert_eq!(add1!(zero_1, _8G), _8G);

    let start = Instant::now();
    let _10H = smul2!(H, nine);                            // H + 9H
    println!("[+] add2 ({:.2?})", start.elapsed());

    assert_eq!(add2!(_10H, zero_2), _10H);
    assert_eq!(add2!(zero_2, _10H), _10H);

    // Check inclusion group
    assert!(contained_in_group!(_8G));                      //  8G E G1
    assert!(contained_in_group!(_10H));                     // 10H E G2

    // Pairing
    let start = Instant::now();
    let res = pair!(_7G, _9H);                              // 7G * 9H
    println!("[+] pair ({:.2?})", start.elapsed());

    // Bytes exports
    
    let z1 = zeroG1!();
    let start = Instant::now();
    bytes1!(z1);
    println!("[+] bytes1 ({:.2?})", start.elapsed());

    let z2 = zeroG2!();
    let start = Instant::now();
    bytes2!(z2);
    println!("[+] bytes2 ({:.2?})", start.elapsed());

    // hash-G1
    use sha2::Digest;
    use std::convert::TryInto;

    use std::iter::FromIterator;
    let bytes: Vec<u8> = (0..5).collect();
    let start = Instant::now();
    hashG1!(&bytes);
    println!("[+] Computed G1-hash ({:.2?})", start.elapsed());
}
