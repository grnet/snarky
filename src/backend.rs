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
macro_rules! rndscalar {
    ($rng:expr) => {
        {
            let mut buf = [0; 64];
            $rng.fill_bytes(&mut buf);
            ::bls12_381::Scalar::from_bytes_wide(&buf)
        }
    }
}

#[macro_export]
macro_rules! pow {
    ($base:expr, $exp:expr) => {
        $base.pow(&[$exp as u64, 0, 0, 0])
    }
}

#[macro_export]
macro_rules! contained_in_group {
    ($elem:expr) => {
        bool::from($elem.is_on_curve())
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
macro_rules! add_1 {
    ($($elem:expr), *) => {
        {
            let mut elems = Vec::new();
            $(elems.push(::bls12_381::G1Projective::from($elem));)*
            ::bls12_381::G1Affine::from(
                elems.iter().sum::<::bls12_381::G1Projective>()
            )
        }
    }
}

#[macro_export]
macro_rules! add_2 {
    ($($elem:expr), *) => {
        {
            let mut elems = Vec::new();
            $(elems.push(::bls12_381::G2Projective::from($elem));)*
            ::bls12_381::G2Affine::from(
                elems.iter().sum::<::bls12_381::G2Projective>()
            )
        }
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
macro_rules! G1_zero {
    () => {
        ::bls12_381::G1Affine::from(
            ::bls12_381::G1Affine::generator() * 
            ::bls12_381::Scalar::zero()
        )
    }
}

#[macro_export]
macro_rules! G2_zero {
    () => {
        ::bls12_381::G2Affine::from(
            ::bls12_381::G2Affine::generator() * 
            ::bls12_381::Scalar::zero()
        )
    }
}

#[macro_export]
macro_rules! pair {
    ($left:expr, $right:expr) => {
        ::bls12_381::pairing(&$left, &$right)
    }
}

#[macro_export]
macro_rules! bytes_1 {
    ($elem:expr) => {
        $elem.to_uncompressed()     // 96 bytes
    }
}

#[macro_export]
macro_rules! bytes_2 {
    ($elem:expr) => {
        $elem.to_compressed()       // 96 bytes
    }
}

#[macro_export]
macro_rules! hashG1 {
    // bytes must be of type &[u8]
    ($bytes:expr) => {
        {
            // Alternative Sha256 version (not working well with bls12_381 scalars)
            //
            // let mut hasher = ::sha2::Sha256::default();
            // hasher.update(bytes);
            // let buffer: [u8; 32] = hasher.finalize().try_into().unwrap();
            // let factor = ::bls12_381::Scalar::from_bytes(&buffer).unwrap();

            let mut hasher = ::sha2::Sha512::default();
            hasher.update($bytes);
            let buffer: [u8; 64] = hasher.finalize().as_slice().try_into().unwrap();
            let factor = ::bls12_381::Scalar::from_bytes_wide(&buffer);

            ::bls12_381::G1Affine::from(::bls12_381::G1Affine::generator() * factor)
        }
    }
}

// Export type aliases to be uniformly used accross the project
pub type Scalar = ::bls12_381::Scalar;
pub type G1Elem = ::bls12_381::G1Affine;
pub type G2Elem = ::bls12_381::G2Affine;

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

    #[test]
    fn test_contained_in_group() {
        let G = G1_gen!();
        let H = G2_gen!();
        let factors = vec![0, 1, 2, 7, 11, 666, 389473847];
        for factor in factors {
            let factor = scalar!(factor);
            assert!(contained_in_group!(mult_1!(G, factor)));
            assert!(contained_in_group!(mult_2!(H, factor)));
        }
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
    fn test_G1_zero() {
        assert_eq!(
            G1Affine::from(G1Affine::generator() * Scalar::zero()), 
            G1_zero!()
        );
    }

    #[test]
    fn test_G2_zero() {
        assert_eq!(
            G2Affine::from(G2Affine::generator() * Scalar::zero()), 
            G2_zero!()
        );
    }

    #[test]
    fn test_add_1() {
        assert_eq!(add_1!(), G1_zero!());

        let G = G1_gen!();
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
        assert_eq!(add_2!(), G2_zero!());

        let H = G2_gen!();
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

    #[test]
    fn test_bytes_1() {
        let mut expected: [u8; 96] = [0; 96];
        expected[0] = 64;
        assert!(bytes_1!(G1_zero!()) == expected);
    }

    #[test]
    fn test_bytes_2() {
        let mut expected: [u8; 96] = [0; 96];
        expected[0] = 192;
        assert!(bytes_2!(G2_zero!()) == expected);
    }
}
