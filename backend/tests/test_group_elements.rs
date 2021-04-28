use backend::*;

use backend::*;
use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_bls12_381;

#[test]
fn test_genG1() {
    assert_eq!(
        ::ark_bls12_381::G1Affine::prime_subgroup_generator(),
        genG1!()
    );
}

#[test]
fn test_genG2() {
    assert_eq!(
        ::ark_bls12_381::G2Affine::prime_subgroup_generator(),
        genG2!()
    );
}

#[test]
fn test_zeroG1() {
    assert_eq!(
        ::ark_bls12_381::G1Affine::zero(),
        zeroG1!()
    );
}

#[test]
fn test_zeroG2() {
    assert_eq!(
        ::ark_bls12_381::G2Affine::zero(),
        zeroG2!()
    );
}

#[test]
fn test_contained_in_group() {
    let G = genG1!();
    let H = genG2!();
    let parametrization = [0u64, 1, 2, 7, 11, 666, 389473847];
    for &factor in &parametrization {
        let f = scalar!(factor);
        let elem1 = smul1!(f, G);
        let elem2 = smul2!(f, H);
        assert!(contained_in_group!(elem1));
        assert!(contained_in_group!(elem2));
    }
}

#[test]
fn test_bytes1() {
    let mut expected: [u8; 97] = [0; 97];
    expected[48] = 1;
    expected[96] = 1;
    assert!(bytes1!(zeroG1!()) == expected);
}

#[test]
fn test_bytes2() {
    let mut expected: [u8; 193] = [0; 193];
    expected[96]  = 1;
    expected[192] = 1;
    assert!(bytes2!(zeroG2!()) == expected);
}

#[test]
fn test_ct_comparisons() {
    let elm1 = scalar!(0u64); 
    let elm2 = scalar!(0u64); 
    let elm3 = scalar!(1u64); 

    // use subtle::ConstantTimeEq;  TODO: Enable when ready for ark_bls12_381

    assert!(ct_eq!(elm1, elm2));
    assert!(ct_ne!(elm1, elm3));

    assert!(!ct_eq!(elm1, elm3));
    assert!(!ct_ne!(elm1, elm2));
}
