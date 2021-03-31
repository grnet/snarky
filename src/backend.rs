#[macro_export]
macro_rules! scalar {
    ($num:expr) => {
        ::bls12_381::Scalar::from($num)
    }
}

#[macro_export]
macro_rules! zero {
    () => {
        ::bls12_381::Scalar::zero()
    }
}

#[macro_export]
macro_rules! one {
    () => {
        ::bls12_381::Scalar::one()
    }
}

#[macro_export]
macro_rules! rand_scalar {
    ($rng:expr) => {
        {
            let mut buf = [0; 64];
            $rng.fill_bytes(&mut buf);
            ::bls12_381::Scalar::from_bytes_wide(&buf)
        }
    }
}

#[macro_export]
macro_rules! G1_gen {
    () => {
        ::bls12_381::G1Affine::generator()
    }
}

#[macro_export]
macro_rules! G2_gen {
    () => {
        ::bls12_381::G2Affine::generator()
    }
}

#[macro_export]
macro_rules! mult_1 {
    ($elem: expr, $factor: expr) => {
        ::bls12_381::G1Affine::from($elem * $factor)
    }
}

#[macro_export]
macro_rules! mult_2 {
    ($elem: expr, $factor: expr) => {
        ::bls12_381::G2Affine::from($elem * $factor)
    }
}

#[macro_export]
macro_rules! pair {
    ($left:expr, $right:expr) => {
        ::bls12_381::pairing(&$left, &$right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;
    use bls12_381::{
        Scalar, G1Affine, G2Affine, pairing,
    };

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
    fn test_G1_gen() {
        assert_eq!(G1Affine::generator(), G1_gen!());
    }

    #[test]
    fn test_G2_gen() {
        assert_eq!(G2Affine::generator(), G2_gen!());
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
}
