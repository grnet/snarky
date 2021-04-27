use sha2::Digest;
use std::convert::TryInto;
use crate::Backend;

#[derive(Debug, PartialEq)]
pub struct RcBls12_381;

impl Backend for RcBls12_381 {

    type Scalar = ::bls12_381::Scalar;
    type G1Elem = ::bls12_381::G1Affine;
    type G2Elem = ::bls12_381::G2Affine;
    type Bytes1 = [u8; 96];
    type Bytes2 = [u8; 96];
    type Gt = ::bls12_381::Gt;

    fn scalar(num: u64) -> Self::Scalar {
        ::bls12_381::Scalar::from(num)
    }

    fn zero() -> Self::Scalar {
        ::bls12_381::Scalar::zero()
    }

    fn one() -> Self::Scalar {
        ::bls12_381::Scalar::one()
    }

    fn rscalar(rng: &mut ::rand::RngCore) -> Self::Scalar {
        let mut buf = [0; 64];
        rng.fill_bytes(&mut buf);
        ::bls12_381::Scalar::from_bytes_wide(&buf)
    }

    fn inv(elem: &Self::Scalar) -> Self::Scalar {
        elem.invert().unwrap()
    }

    fn pow(base: &Self::Scalar, exp: usize) -> Self::Scalar {
        base.pow(&[exp as u64, 0, 0, 0])
    }

    fn genG1() -> Self::G1Elem {
        ::bls12_381::G1Affine::generator()
    }

    fn genG2() -> Self::G2Elem {
        ::bls12_381::G2Affine::generator()
    }

    fn zeroG1() -> Self::G1Elem {
        ::bls12_381::G1Affine::default()
    }

    fn zeroG2() -> Self::G2Elem {
        ::bls12_381::G2Affine::default()
    }

    fn contained_in_G1(elem: &Self::G1Elem) -> bool {
        bool::from(elem.is_on_curve())
    }

    fn contained_in_G2(elem: &Self::G2Elem) -> bool {
        bool::from(elem.is_on_curve())
    }

    fn bytes1(elem: &Self::G1Elem) -> Self::Bytes1 {
        elem.to_uncompressed()
    }

    fn bytes2(elem: &Self::G2Elem) -> Self::Bytes2 {
        elem.to_compressed()
    }

    fn add1(elems: &[Self::G1Elem]) -> Self::G1Elem {
        ::bls12_381::G1Affine::from(
            elems
                .iter()
                .map(|elem| ::bls12_381::G1Projective::from(elem))
                .sum::<::bls12_381::G1Projective>()
        )
    }

    fn add2(elems: &[Self::G2Elem]) -> Self::G2Elem {
        ::bls12_381::G2Affine::from(
            elems
                .iter()
                .map(|elem| ::bls12_381::G2Projective::from(elem))
                .sum::<::bls12_381::G2Projective>()
        )
    }

    fn smul1(factor: &Self::Scalar, elem: &Self::G1Elem) -> Self::G1Elem {
        ::bls12_381::G1Affine::from(elem * factor)
    }

    fn smul2(factor: &Self::Scalar, elem: &Self::G2Elem) -> Self::G2Elem {
        ::bls12_381::G2Affine::from(elem * factor)
    }

    fn pair(left: &Self::G1Elem, right: &Self::G2Elem) -> Self::Gt {
        ::bls12_381::pairing(left, right)
    }

    fn hashG1(bytes: &[u8]) -> Self::G1Elem {
        // Alternative Sha256 version (not working well with bls12_381 scalars)
        //
        // let mut hasher = ::sha2::Sha256::default();
        // hasher.update(bytes);
        // let buffer: [u8; 32] = hasher.finalize().try_into().unwrap();
        // let factor = ::bls12_381::Scalar::from_bytes(&buffer).unwrap();

        let mut hasher = ::sha2::Sha512::default();
        hasher.update(bytes);
        let buffer: [u8; 64] = hasher.finalize().as_slice().try_into().unwrap();
        let factor = ::bls12_381::Scalar::from_bytes_wide(&buffer);

        ::bls12_381::G1Affine::from(::bls12_381::G1Affine::generator() * factor)
    }
}

#[cfg(test)]
mod tests {

    use util::*;
    use super::RcBls12_381;
    use crate::Backend;

    #[test]
    fn test_scalar() {
        let parametrization = [0, 1, 2, 10, 666];
        for &num in &parametrization {
            assert_eq!(
                ::bls12_381::Scalar::from(num), 
                RcBls12_381::scalar(num)
            );
        }
    }

    #[test]
    fn test_zero() {
        assert_eq!(::bls12_381::Scalar::from(0), RcBls12_381::zero());
        assert_ne!(::bls12_381::Scalar::from(1), RcBls12_381::zero());
    }

    #[test]
    fn test_one() {
        assert_eq!(::bls12_381::Scalar::from(1), RcBls12_381::one());
        assert_ne!(::bls12_381::Scalar::from(0), RcBls12_381::one());
    }

    #[test]
    fn test_inv() {
        let parametrization = vec![1, 2, 10, 666];
        for num in parametrization {
            let s = RcBls12_381::scalar(num);
            assert_eq!(
                s.invert().unwrap(),
                RcBls12_381::inv(&s),
            );
        }
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
            assert_eq!(
                RcBls12_381::pow(&RcBls12_381::scalar(base), exp), 
                RcBls12_381::scalar(result)
            );
        }
    }

    #[test]
    fn test_genG1() {
        assert_eq!(
            ::bls12_381::G1Affine::generator(),
            RcBls12_381::genG1()
        );
    }

    #[test]
    fn test_genG2() {
        assert_eq!(
            ::bls12_381::G2Affine::generator(),
            RcBls12_381::genG2()
        );
    }

    #[test]
    fn test_zeroG1() {
        assert_eq!(
            ::bls12_381::G1Affine::default(),
            RcBls12_381::zeroG1()
        );
    }
    
    #[test]
    fn test_zeroG2() {
        assert_eq!(
            ::bls12_381::G2Affine::default(),
            RcBls12_381::zeroG2()
        );
    }

    #[test]
    fn test_contained_in_G1() {
        let G = RcBls12_381::genG1();
        let parametrization = [0, 1, 2, 7, 11, 666, 389473847];
        for factor in &parametrization {
            let f = RcBls12_381::scalar(*factor);
            let elem = RcBls12_381::smul1(&f, &G);
            assert!(RcBls12_381::contained_in_G1(&elem));
        }
    }

    #[test]
    fn test_contained_in_G2() {
        let H = RcBls12_381::genG1();
        let parametrization = [0, 1, 2, 7, 11, 666, 389473847];
        for factor in &parametrization {
            let f = RcBls12_381::scalar(*factor);
            let elem = RcBls12_381::smul1(&f, &H);
            assert!(RcBls12_381::contained_in_G1(&elem));
        }
    }

    #[test]
    fn test_bytes1() {
        let mut expected: [u8; 96] = [0; 96];
        expected[0] = 64;
        assert_eq!(
            RcBls12_381::bytes1(&RcBls12_381::zeroG1()),
            expected
        );
    }

    #[test]
    fn test_bytes2() {
        let mut expected: [u8; 96] = [0; 96];
        expected[0] = 192;
        assert_eq!(
            RcBls12_381::bytes2(&RcBls12_381::zeroG2()),
            expected
        );
    }

    #[test]
    fn test_ct_eq() {
        let elm1 = RcBls12_381::scalar(0); 
        let elm2 = RcBls12_381::scalar(0); 
        let elm3 = RcBls12_381::scalar(1); 
    
        assert!(RcBls12_381::ct_eq(&elm1, &elm2));
        assert!(!RcBls12_381::ct_eq(&elm1, &elm3));
    }

    #[test]
    fn test_ct_ne() {
        let elm1 = RcBls12_381::scalar(0); 
        let elm2 = RcBls12_381::scalar(0); 
        let elm3 = RcBls12_381::scalar(1); 
    
        assert!(RcBls12_381::ct_ne(&elm1, &elm3));
        assert!(!RcBls12_381::ct_ne(&elm1, &elm2));
    }

    #[test]
    fn test_add1() {
        let G = RcBls12_381::genG1();
        assert_eq!(RcBls12_381::add1(&[]), RcBls12_381::zeroG1());
        assert_eq!(RcBls12_381::add1(&[G]), G);
        assert_eq!(
            RcBls12_381::add1(&[G, G]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(2), &G));
        assert_eq!(
            RcBls12_381::add1(&[G, G, G]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(3), &G));
        assert_eq!(
            RcBls12_381::add1(&[RcBls12_381::smul1(&RcBls12_381::scalar(2), &G), G]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(3), &G));
        assert_eq!(
            RcBls12_381::add1(&[G, RcBls12_381::smul1(&RcBls12_381::scalar(2), &G)]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(3), &G));
        assert_eq!(
            RcBls12_381::add1(&[G, G, G, G]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(4), &G));
        assert_eq!(
            RcBls12_381::add1(&[RcBls12_381::smul1(&RcBls12_381::scalar(3), &G), G]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(4), &G));
        assert_eq!(
            RcBls12_381::add1(&[G, RcBls12_381::smul1(&RcBls12_381::scalar(3), &G)]), 
            RcBls12_381::smul1(&RcBls12_381::scalar(4), &G));
        assert_eq!(
            RcBls12_381::add1(&[
                RcBls12_381::smul1(&RcBls12_381::scalar(2), &G), 
                RcBls12_381::smul1(&RcBls12_381::scalar(2), &G)]),
            RcBls12_381::smul1(&RcBls12_381::scalar(4), &G));
    }

    #[test]
    fn test_add2() {
        let H = RcBls12_381::genG2();
        assert_eq!(RcBls12_381::add2(&[]), RcBls12_381::zeroG2());
        assert_eq!(RcBls12_381::add2(&[H]), H);
        assert_eq!(
            RcBls12_381::add2(&[H, H]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(2), &H));
        assert_eq!(
            RcBls12_381::add2(&[H, H, H]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(3), &H));
        assert_eq!(
            RcBls12_381::add2(&[RcBls12_381::smul2(&RcBls12_381::scalar(2), &H), H]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(3), &H));
        assert_eq!(
            RcBls12_381::add2(&[H, RcBls12_381::smul2(&RcBls12_381::scalar(2), &H)]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(3), &H));
        assert_eq!(
            RcBls12_381::add2(&[H, H, H, H]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(4), &H));
        assert_eq!(
            RcBls12_381::add2(&[RcBls12_381::smul2(&RcBls12_381::scalar(3), &H), H]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(4), &H));
        assert_eq!(
            RcBls12_381::add2(&[H, RcBls12_381::smul2(&RcBls12_381::scalar(3), &H)]), 
            RcBls12_381::smul2(&RcBls12_381::scalar(4), &H));
        assert_eq!(
            RcBls12_381::add2(&[
                RcBls12_381::smul2(&RcBls12_381::scalar(2), &H), 
                RcBls12_381::smul2(&RcBls12_381::scalar(2), &H)]),
            RcBls12_381::smul2(&RcBls12_381::scalar(4), &H));
    }

    #[test]
    fn test_smul1() {
        let parametrization = [
            (0, 0), (0, 1), (0, 2), (0, 100),
            (1, 0), (1, 1), (1, 2), (1, 100),
            (2, 0), (2, 1), (2, 2), (2, 100),
            (7, 0), (7, 1), (7, 2), (7, 100),
            (9, 0), (9, 1), (9, 2), (9, 100),
            (666, 0), (666, 1), (666, 2), (666, 100),
        ];
        for &(f1, f2) in &parametrization {
    
            let a = RcBls12_381::smul1(
                &RcBls12_381::scalar(f1), &RcBls12_381::genG1());       // f1 * G
            let b = RcBls12_381::smul1(
                &RcBls12_381::scalar(f2), &a);                          // f2 * f1 * G
    
            assert_eq!(
                ::bls12_381::G1Affine::from(
                    ::bls12_381::G1Affine::generator() * ::bls12_381::Scalar::from(f1)),
                a);
            assert_eq!(
                ::bls12_381::G1Affine::from(a * ::bls12_381::Scalar::from(f2)),
                b);
        }
    }

    #[test]
    fn test_smul2() {
        let parametrization = [
            (0, 0), (0, 1), (0, 2), (0, 100),
            (1, 0), (1, 1), (1, 2), (1, 100),
            (2, 0), (2, 1), (2, 2), (2, 100),
            (7, 0), (7, 1), (7, 2), (7, 100),
            (9, 0), (9, 1), (9, 2), (9, 100),
            (666, 0), (666, 1), (666, 2), (666, 100),
        ];
        for &(f1, f2) in &parametrization {
    
            let a = RcBls12_381::smul2(
                &RcBls12_381::scalar(f1), &RcBls12_381::genG2());       // f1 * H
            let b = RcBls12_381::smul2(
                &RcBls12_381::scalar(f2), &a);                          // f2 * f1 * H
    
            assert_eq!(
                ::bls12_381::G2Affine::from(
                    ::bls12_381::G2Affine::generator() * ::bls12_381::Scalar::from(f1)),
                a);
            assert_eq!(
                ::bls12_381::G2Affine::from(a * ::bls12_381::Scalar::from(f2)),
                b);
        }
    }

    #[test]
    fn test_pair() {
        let parametrization = [
            (0, 0), (0, 1), (1, 0), (1, 1),
            (1, 2), (2, 1), (2, 2), (3, 2),
            (2, 3), (3, 3), (4, 3), (3, 4),
            (666, 999), (999, 666), (666, 0), (0, 666),
        ];
        for &(f1, f2) in &parametrization {
            let left  = RcBls12_381::smul1(&RcBls12_381::scalar(f1), &RcBls12_381::genG1());
            let right = RcBls12_381::smul2(&RcBls12_381::scalar(f2), &RcBls12_381::genG2());
            assert_eq!(RcBls12_381::pair(&left, &right), ::bls12_381::pairing(&left, &right));
        }
    }
}
