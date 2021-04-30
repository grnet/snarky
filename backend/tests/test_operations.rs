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
fn test_add1() {
    let G = genG1!();
    assert_eq!(add1!(), zeroG1!());
    assert_eq!(add1!(G), G);
    assert_eq!(add1!(G, G), smul1!(scalar!(2u64), G));
    assert_eq!(add1!(G, G, G), smul1!(scalar!(3u64), G));
    assert_eq!(add1!(smul1!(scalar!(2u64), G), G), smul1!(scalar!(3u64), G));
    assert_eq!(add1!(G, smul1!(scalar!(2u64), G)), smul1!(scalar!(3u64), G));
    assert_eq!(add1!(G, G, G, G), smul1!(scalar!(4u64), G));
    assert_eq!(add1!(smul1!(scalar!(3u64), G), G), smul1!(scalar!(4u64), G));
    assert_eq!(add1!(G, smul1!(scalar!(3u64), G)), smul1!(scalar!(4u64), G));
    assert_eq!(add1!(smul1!(scalar!(2u64), G), smul1!(scalar!(2u64), G)), smul1!(scalar!(4u64), G));
}

#[test]
fn test_add2() {
    let H = genG2!();
    assert_eq!(add2!(), zeroG2!());
    assert_eq!(add2!(H), H);
    assert_eq!(add2!(H, H), smul2!(scalar!(2u64), H));
    assert_eq!(add2!(H, H, H), smul2!(scalar!(3u64), H));
    assert_eq!(add2!(smul2!(scalar!(2u64), H), H), smul2!(scalar!(3u64), H));
    assert_eq!(add2!(H, smul2!(scalar!(2u64), H)), smul2!(scalar!(3u64), H));
    assert_eq!(add2!(H, H, H, H), smul2!(scalar!(4u64), H));
    assert_eq!(add2!(smul2!(scalar!(3u64), H), H), smul2!(scalar!(4u64), H));
    assert_eq!(add2!(H, smul2!(scalar!(3u64), H)), smul2!(scalar!(4u64), H));
    assert_eq!(add2!(smul2!(scalar!(2u64), H), smul2!(scalar!(2u64), H)), smul2!(scalar!(4u64), H));
}

#[test]
fn test_smul1() {
    let parametrization: Vec<(u64, u64)> = vec! [
        (0, 0), (0, 1), (0, 2), (0, 100),
        (1, 0), (1, 1), (1, 2), (1, 100),
        (2, 0), (2, 1), (2, 2), (2, 100),
        (7, 0), (7, 1), (7, 2), (7, 100),
        (9, 0), (9, 1), (9, 2), (9, 100),
        (666, 0), (666, 1), (666, 2), (666, 100),
    ];
    for (f1, f2) in parametrization {

        let a = smul1!(scalar!(f1), genG1!());  // f1 * G
        let b = smul1!(scalar!(f2), a);         // f2 * f1 * G

        assert_eq!(
            ::ark_bls12_381::G1Affine::from(
                ::ark_bls12_381::G1Affine::prime_subgroup_generator().mul(scalar!(f1))
            ),
            a
        );
        assert_eq!(
            ::ark_bls12_381::G1Affine::from(a.mul(scalar!(f2))),
            b
        );
    }
}

#[test]
fn test_smul2() {
    let parametrization: Vec<(u64, u64)> = vec! [
        (0, 0), (0, 1), (0, 2), (0, 100),
        (1, 0), (1, 1), (1, 2), (1, 100),
        (2, 0), (2, 1), (2, 2), (2, 100),
        (7, 0), (7, 1), (7, 2), (7, 100),
        (9, 0), (9, 1), (9, 2), (9, 100),
        (666, 0), (666, 1), (666, 2), (666, 100),
    ];
    for (f1, f2) in parametrization {

        let a = smul2!(scalar!(f1), genG2!());  // f1 * H
        let b = smul2!(scalar!(f2), a);         // f2 * f1 * H

        assert_eq!(
            ::ark_bls12_381::G2Affine::from(
                ::ark_bls12_381::G2Affine::prime_subgroup_generator().mul(scalar!(f1))
            ),
            a
        );
        assert_eq!(
            ::ark_bls12_381::G2Affine::from(a.mul(scalar!(f2))),
            b
        );
    }
}

#[test]
fn test_pair() {
    let parametrization: Vec<(u64, u64)> = vec! [
        (0, 0), (0, 1), (1, 0), (1, 1),
        (1, 2), (2, 1), (2, 2), (3, 2),
        (2, 3), (3, 3), (4, 3), (3, 4),
        (666, 999), (999, 666), (666, 0), (0, 666),
    ];
    for (f1, f2) in parametrization {
        let left  = smul1!(scalar!(f1), genG1!());
        let right = smul2!(scalar!(f2), genG2!());
        assert_eq!(
            ::ark_bls12_381::Bls12_381::pairing(left, right),
            pair!(left, right)
        );
    }
}
