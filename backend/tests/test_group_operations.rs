use backend::{
    add1, add2, smul1, smul2, pair,
};
use backend::{scalar, genG1, genG2, zeroG1, zeroG2};
use bls12_381::{Scalar, G1Affine, G2Affine, pairing};

#[test]
fn test_add1() {
    let G = genG1!();
    assert_eq!(add1!(), zeroG1!());
    assert_eq!(add1!(G), G);
    assert_eq!(add1!(G, G), smul1!(G, scalar!(2)));
    assert_eq!(add1!(G, G, G), smul1!(G, scalar!(3)));
    assert_eq!(add1!(smul1!(G, scalar!(2)), G), smul1!(G, scalar!(3)));
    assert_eq!(add1!(G, smul1!(G, scalar!(2))), smul1!(G, scalar!(3)));
    assert_eq!(add1!(G, G, G, G), smul1!(G, scalar!(4)));
    assert_eq!(add1!(smul1!(G, scalar!(3)), G), smul1!(G, scalar!(4)));
    assert_eq!(add1!(G, smul1!(G, scalar!(3))), smul1!(G, scalar!(4)));
    assert_eq!(add1!(smul1!(G, scalar!(2)), smul1!(G, scalar!(2))), smul1!(G, scalar!(4)));
}

#[test]
fn test_add2() {
    let H = genG2!();
    assert_eq!(add2!(), zeroG2!());
    assert_eq!(add2!(H), H);
    assert_eq!(add2!(H, H), smul2!(H, scalar!(2)));
    assert_eq!(add2!(H, H, H), smul2!(H, scalar!(3)));
    assert_eq!(add2!(smul2!(H, scalar!(2)), H), smul2!(H, scalar!(3)));
    assert_eq!(add2!(H, smul2!(H, scalar!(2))), smul2!(H, scalar!(3)));
    assert_eq!(add2!(H, H, H, H), smul2!(H, scalar!(4)));
    assert_eq!(add2!(smul2!(H, scalar!(3)), H), smul2!(H, scalar!(4)));
    assert_eq!(add2!(H, smul2!(H, scalar!(3))), smul2!(H, scalar!(4)));
    assert_eq!(add2!(smul2!(H, scalar!(2)), smul2!(H, scalar!(2))), smul2!(H, scalar!(4)));
}

#[test]
fn test_smul1() {
    let parametrization = vec! [
        (0, 0), (0, 1), (0, 2), (0, 100),
        (1, 0), (1, 1), (1, 2), (1, 100),
        (2, 0), (2, 1), (2, 2), (2, 100),
        (7, 0), (7, 1), (7, 2), (7, 100),
        (9, 0), (9, 1), (9, 2), (9, 100),
        (666, 0), (666, 1), (666, 2), (666, 100),
    ];
    for (f1, f2) in parametrization {

        let a = smul1!(genG1!(), scalar!(f1));        // f1 * G
        let b = smul1!(a, scalar!(f2));                // f2 * f1 * G

        assert_eq!(a, G1Affine::from(G1Affine::generator() * Scalar::from(f1)));
        assert_eq!(b, G1Affine::from(a * Scalar::from(f2)));
    }
}

#[test]
fn test_smul2() {
    let parametrization = vec! [
        (0, 0), (0, 1), (0, 2), (0, 100),
        (1, 0), (1, 1), (1, 2), (1, 100),
        (2, 0), (2, 1), (2, 2), (2, 100),
        (7, 0), (7, 1), (7, 2), (7, 100),
        (9, 0), (9, 1), (9, 2), (9, 100),
        (666, 0), (666, 1), (666, 2), (666, 100),
    ];
    for (f1, f2) in parametrization {

        let a = smul2!(genG2!(), scalar!(f1));        // f1 * H
        let b = smul2!(a, scalar!(f2));                // f2 * f1 * H

        assert_eq!(a, G2Affine::from(G2Affine::generator() * Scalar::from(f1)));
        assert_eq!(b, G2Affine::from(a * Scalar::from(f2)));
    }
}

#[test]
fn test_pair() {
    let parametrization = vec! [
        (0, 0), (0, 1), (1, 0), (1, 1),
        (1, 2), (2, 1), (2, 2), (3, 2),
        (2, 3), (3, 3), (4, 3), (3, 4),
        (666, 999), (999, 666), (666, 0), (0, 666),
    ];
    for (f1, f2) in parametrization {
        let left  = smul1!(genG1!(), scalar!(f1));
        let right = smul2!(genG2!(), scalar!(f2));
        assert_eq!(pair!(left, right), pairing(&left, &right));
    }
}
