use backend::{
    add_1, add_2, mult_1, mult_2, pair,
};
use backend::{scalar, G1_gen, G2_gen, G1_zero, G2_zero};
use bls12_381::{Scalar, G1Affine, G2Affine, pairing};

#[test]
fn test_add_1() {
    let G = G1_gen!();
    assert_eq!(add_1!(), G1_zero!());
    assert_eq!(add_1!(G), G);
    assert_eq!(add_1!(G, G), mult_1!(G, scalar!(2)));
    assert_eq!(add_1!(G, G, G), mult_1!(G, scalar!(3)));
    assert_eq!(add_1!(mult_1!(G, scalar!(2)), G), mult_1!(G, scalar!(3)));
    assert_eq!(add_1!(G, mult_1!(G, scalar!(2))), mult_1!(G, scalar!(3)));
    assert_eq!(add_1!(G, G, G, G), mult_1!(G, scalar!(4)));
    assert_eq!(add_1!(mult_1!(G, scalar!(3)), G), mult_1!(G, scalar!(4)));
    assert_eq!(add_1!(G, mult_1!(G, scalar!(3))), mult_1!(G, scalar!(4)));
    assert_eq!(add_1!(mult_1!(G, scalar!(2)), mult_1!(G, scalar!(2))), mult_1!(G, scalar!(4)));
}

#[test]
fn test_add_2() {
    let H = G2_gen!();
    assert_eq!(add_2!(), G2_zero!());
    assert_eq!(add_2!(H), H);
    assert_eq!(add_2!(H, H), mult_2!(H, scalar!(2)));
    assert_eq!(add_2!(H, H, H), mult_2!(H, scalar!(3)));
    assert_eq!(add_2!(mult_2!(H, scalar!(2)), H), mult_2!(H, scalar!(3)));
    assert_eq!(add_2!(H, mult_2!(H, scalar!(2))), mult_2!(H, scalar!(3)));
    assert_eq!(add_2!(H, H, H, H), mult_2!(H, scalar!(4)));
    assert_eq!(add_2!(mult_2!(H, scalar!(3)), H), mult_2!(H, scalar!(4)));
    assert_eq!(add_2!(H, mult_2!(H, scalar!(3))), mult_2!(H, scalar!(4)));
    assert_eq!(add_2!(mult_2!(H, scalar!(2)), mult_2!(H, scalar!(2))), mult_2!(H, scalar!(4)));
}

#[test]
fn test_mult_1() {
    let parametrization = vec! [
        (0, 0), (0, 1), (0, 2), (0, 100),
        (1, 0), (1, 1), (1, 2), (1, 100),
        (2, 0), (2, 1), (2, 2), (2, 100),
        (7, 0), (7, 1), (7, 2), (7, 100),
        (9, 0), (9, 1), (9, 2), (9, 100),
        (666, 0), (666, 1), (666, 2), (666, 100),
    ];
    for (f1, f2) in parametrization {

        let a = mult_1!(G1_gen!(), scalar!(f1));        // f1 * G
        let b = mult_1!(a, scalar!(f2));                // f2 * f1 * G

        assert_eq!(a, G1Affine::from(G1Affine::generator() * Scalar::from(f1)));
        assert_eq!(b, G1Affine::from(a * Scalar::from(f2)));
    }
}

#[test]
fn test_mult_2() {
    let parametrization = vec! [
        (0, 0), (0, 1), (0, 2), (0, 100),
        (1, 0), (1, 1), (1, 2), (1, 100),
        (2, 0), (2, 1), (2, 2), (2, 100),
        (7, 0), (7, 1), (7, 2), (7, 100),
        (9, 0), (9, 1), (9, 2), (9, 100),
        (666, 0), (666, 1), (666, 2), (666, 100),
    ];
    for (f1, f2) in parametrization {

        let a = mult_2!(G2_gen!(), scalar!(f1));        // f1 * H
        let b = mult_2!(a, scalar!(f2));                // f2 * f1 * H

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
        let left  = mult_1!(G1_gen!(), scalar!(f1));
        let right = mult_2!(G2_gen!(), scalar!(f2));
        assert_eq!(pair!(left, right), pairing(&left, &right));
    }
}
