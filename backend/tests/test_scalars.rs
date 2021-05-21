use backend::*;
use util::map;

use backend::*;
use num_traits::identities::Zero;
use num_traits::identities::One;
use ark_ff::fields::Field;

#[test]
fn test_scalar() {
    let parametrization = [0u64, 1, 2, 10, 666];
    for &num in &parametrization {
        assert_eq!(::ark_bls12_381::Fr::from(num), scalar!(num));
    }
}

#[test]
fn test_zero() {
    assert_eq!(::ark_bls12_381::Fr::from(0u64), zero!());
    assert_ne!(::ark_bls12_381::Fr::from(1u64), zero!());
}

#[test]
fn test_one() {
    assert_eq!(::ark_bls12_381::Fr::from(1u64), one!());
    assert_ne!(::ark_bls12_381::Fr::from(0u64), one!());
}

#[test]
fn test_pow() {
    let parametrization = map! {
        (0, 0) => 1, (1, 0) => 1, (2, 0) => 1, (3, 0) =>  1, (7, 0) =>   1,
        (0, 1) => 0, (1, 1) => 1, (2, 1) => 2, (3, 1) =>  3, (7, 1) =>   7,
        (0, 2) => 0, (1, 2) => 1, (2, 2) => 4, (3, 2) =>  9, (7, 2) =>  49,
        (0, 3) => 0, (1, 3) => 1, (2, 3) => 8, (3, 3) => 27, (7, 3) => 343,
        (0u64, 4u64) => 0u64
    };
    for ((base, exp), result) in parametrization {
        assert_eq!(pow!(scalar!(base), exp), scalar!(result));
    }
}
