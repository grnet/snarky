use backend::{scalar, zero, one, pow};
use bls12_381::Scalar;
use util::map;

#[test]
fn test_scalar() {
    let parametrization = vec![0, 1, 666];
    for value in parametrization {
        assert_eq!(Scalar::from(value), scalar!(value));
    }
}

#[test]
fn test_zero() {
    assert_eq!(Scalar::from(0), zero!());
    assert_ne!(Scalar::from(1), zero!());
}

#[test]
fn test_one() {
    assert_eq!(Scalar::from(1), one!());
    assert_ne!(Scalar::from(0), one!());
}

#[test]
fn test_pow() {
    let parametrization = map! {
        (0, 0) => 1, (1, 0) => 1, (2, 0) => 1, (3, 0) =>  1, (7, 0) =>   1,
        (0, 1) => 0, (1, 1) => 1, (2, 1) => 2, (3, 1) =>  3, (7, 1) =>   7,
        (0, 2) => 0, (1, 2) => 1, (2, 2) => 4, (3, 2) =>  9, (7, 2) =>  49,
        (0, 3) => 0, (1, 3) => 1, (2, 3) => 8, (3, 3) => 27, (7, 3) => 343,
        (0, 4) => 0
    };
    for ((base, exp), result) in parametrization {
        assert_eq!(pow!(scalar!(base), exp), scalar!(result));
    }
}
