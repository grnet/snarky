use std::time::Instant;

use backend::*;
use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
// use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_bls12_381;

fn main() {

    // Scalar definition
    let start = Instant::now();
    let zero = zero!();
    println!("[+] zero ({:.2?})", start.elapsed());

    let start = Instant::now();
    let one = one!();
    println!("[+] one ({:.2?})", start.elapsed());
  
    let start = Instant::now();
    let seven = scalar!(7_u64);
    println!("[+] seven ({:.2?})", start.elapsed());

    let start = Instant::now();
    let nine = scalar!(9_u64);
    println!("[+] nine ({:.2?})", start.elapsed());

    let start = Instant::now();
    let beast = scalar!(666_u64);
    println!("[+] beast ({:.2?})", start.elapsed());

    // Random scalar generation

    use ark_std::rand::Rng;
    use ark_std::rand::RngCore as ArkRngCore;
    use ark_std::rand::SeedableRng;
    let start = Instant::now();
    let r = rscalar!(::util::snarky_rng());
    println!("[+] rscalar ({:.2?})", start.elapsed());

    // Raising scalars to power

    let base = ark_bls12_381::Fr::from(666_u64);
    let exp  = 999;
    let start = Instant::now();
    pow!(base, exp);
    println!("[+] ppow ({:.2?})", start.elapsed());

    // Scalar inversion

    let beast = scalar!(666_u64);
    let start = Instant::now();
    inv!(beast);
    println!("[+] inv ({:.2?})", start.elapsed());

    // Generators

    let start = Instant::now();
    let G = genG1!();                                      // G1 generator
    println!("[+] G ({:.2?})", start.elapsed());

    let start = Instant::now();
    let H = genG2!();                                      // G2 generator
    println!("[+] H ({:.2?})", start.elapsed());

    // Zero group elements
    
    let start = Instant::now();
    let zero_1 = zeroG1!();
    println!("[+] zero G1 ({:.2?})", start.elapsed());

    let start = Instant::now();
    let zero_2 = zeroG2!();
    println!("[+] zero G2 ({:.2?})", start.elapsed());

    // Multiplication

    let start = Instant::now();
    let _7G = smul1!(seven, G);                            // 7G
    println!("[+] smul1 ({:.2?})", start.elapsed());

    let start = Instant::now();
    let _9H = smul2!(nine, H);                             // 9H
    println!("[+] smul2 ({:.2?})", start.elapsed());

    // Addition

    let start = Instant::now();
    let _8G = add1!(G, _7G);                               // G + 7G
    println!("[+] add1 ({:.2?})", start.elapsed());

    assert_eq!(add1!(_8G, zero_1), _8G);
    assert_eq!(add1!(zero_1, _8G), _8G);

    let start = Instant::now();
    let _10H = smul2!(nine, H);                            // H + 9H
    println!("[+] add2 ({:.2?})", start.elapsed());

    assert_eq!(add2!(_10H, zero_2), _10H);
    assert_eq!(add2!(zero_2, _10H), _10H);

    assert_eq!(add2!(_10H, zero_2), _10H);
    assert_eq!(add2!(zero_2, _10H), _10H);

    // Check inclusion group
    
    assert!(contained_in_group!(_8G));                      //  8G E G1
    assert!(contained_in_group!(_10H));                     // 10H E G2

    let start = Instant::now();
    contained_in_group!(G);
    println!("[+] check G1 ({:.2?})", start.elapsed());

    let start = Instant::now();
    contained_in_group!(H);
    println!("[+] check G2 ({:.2?})", start.elapsed());


    // Pairing

    let start = Instant::now();
    let res = pair!(_7G, _9H);                              // 7G * 9H
    println!("[+] pair ({:.2?})", start.elapsed());

    // Bytes exports

    let z1 = zeroG1!();
    let start = Instant::now();
    println!("[+] bytes1 ({:.2?})", start.elapsed());

    let z2 = zeroG2!();
    let start = Instant::now();
    bytes2!(z2);
    println!("[+] bytes2 ({:.2?})", start.elapsed());

    // hash-G1

    use sha2::Digest;
    use std::convert::TryInto;
    use ark_ff::FromBytes;
    use std::io::Cursor;

    let bytes: Vec<u8> = (0..5).collect();

    let start = Instant::now();
    hashG1!(&bytes);
    println!("[+] Computed G1-hhash ({:.2?})", start.elapsed());

    // // Constant-time comparisons
    // use subtle::ConstantTimeEq;

    // let elm1 = scalar!(0); 
    // let elm2 = scalar!(0); 
    // let elm3 = scalar!(1); 

    // assert!(ct_eq!(elm1, elm2));
    // assert!(ct_ne!(elm1, elm3));

    // assert!(!ct_eq!(elm1, elm3));
    // assert!(!ct_ne!(elm1, elm2));
}
