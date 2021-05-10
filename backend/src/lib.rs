pub type Scalar = ::ark_bls12_381::Fr;
pub type G1Elem = ::ark_bls12_381::G1Affine;
pub type G2Elem = ::ark_bls12_381::G2Affine;


#[macro_export]
macro_rules! scalar {
    ($num:expr) => {
        ::ark_bls12_381::Fr::from($num)
    }
}

#[macro_export]
macro_rules! zero {
    () => {
        ::ark_bls12_381::Fr::zero()
    }
}

#[macro_export]
macro_rules! one {
    () => {
        ::ark_bls12_381::Fr::one()
    }
}

#[macro_export]
macro_rules! rscalar {
    ($rng:expr) => {
        {
            let rnd: ark_bls12_381::Fr = $rng.gen(); 
            rnd
        }
    }
}

#[macro_export]
macro_rules! genG1 {
    () => {
        ark_bls12_381::G1Affine::prime_subgroup_generator()
    }
}

#[macro_export]
macro_rules! genG2 {
    () => {
        ark_bls12_381::G2Affine::prime_subgroup_generator()
    }
}

#[macro_export]
macro_rules! zeroG1 {
    () => {
        ::ark_bls12_381::G1Affine::zero()
    }
}

#[macro_export]
macro_rules! zeroG2 {
    () => {
        ::ark_bls12_381::G2Affine::zero()
    }
}

#[macro_export]
macro_rules! unit {
    () => {
        ::ark_bls12_381::Fq12::one()
    }
}

#[macro_export]
macro_rules! contained_in_group {
    ($elem:expr) => {
        bool::from($elem.is_on_curve())
    }
}

#[macro_export]
macro_rules! bytes1 {
    ($elem:expr) => {
        {
            let mut buffer = Vec::<u8>::with_capacity(97);
            $elem.write(&mut buffer);
            buffer  // 97 bytes
        }
    }
}

#[macro_export]
macro_rules! bytes2 {
    ($elem:expr) => {
        {
            let mut buffer = Vec::<u8>::with_capacity(97);
            $elem.write(&mut buffer);
            buffer  // 97 bytes
        }
    }
}

// TODO: ct_eq, ct_ne

#[macro_export]
// Constant-time equality check
// Note: Applies to all types of elements for bls12_381. The
// bls12_831 backend uses subtle for contant-time operations:
// https://docs.rs/subtle/2.4.0/subtle/
macro_rules! ct_eq {
    ($elem1:expr, $elem2:expr) => {
        // https://docs.rs/subtle/2.4.0/src/subtle/lib.rs.html#67
        // bool::from($elem1.ct_eq(&$elem2))
        // TODO: Properly implement when CT interface is ready for ark_bls12_381
        $elem1 == $elem2
    }
}

#[macro_export]
// Constant-time inequality check
// Note: Applies to all types of elements for bls12_381. The
// bls12_831 backend uses subtle for contant-time operations:
// https://docs.rs/subtle/2.4.0/subtle/
macro_rules! ct_ne {
    ($elem1:expr, $elem2:expr) => {
        // https://docs.rs/subtle/2.4.0/src/subtle/lib.rs.html#67
        // !bool::from($elem1.ct_eq(&$elem2))
        // TODO: Properly implement when CT interface is ready for ark_bls12_381
        $elem1 != $elem2
    }
}

#[macro_export]
macro_rules! inv {
    ($elem:expr) => {
       $elem.inverse().unwrap() 
    }
}

#[macro_export]
macro_rules! pow {
    ($base:expr, $exp:expr) => {
        $base.pow(&[$exp as u64, 0, 0, 0])
    }
}

#[macro_export]
macro_rules! add1 {
    ($($elem:expr), *) => {
        {
            let mut sum = ::ark_bls12_381::G1Affine::zero();
            $(sum += &$elem;)*
            sum
        }
    }
}

#[macro_export]
macro_rules! add2 {
    ($($elem:expr), *) => {
        {
            let mut sum = ::ark_bls12_381::G2Affine::zero();
            $(sum += &$elem;)*
            sum
        }
    }
}

#[macro_export]
macro_rules! smul1 {
    ($factor: expr, $elem: expr) => {
        ::ark_bls12_381::G1Affine::from($elem.mul($factor));
    }
}

#[macro_export]
macro_rules! smul2 {
    ($factor: expr, $elem: expr) => {
        ::ark_bls12_381::G2Affine::from($elem.mul($factor));
    }
}

#[macro_export]
macro_rules! pair {
    ($left:expr, $right:expr) => {
        ::ark_bls12_381::Bls12_381::pairing($left, $right)
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
            // hasher.update($bytes);
            // let buffer: [u8; 32] = hasher.finalize().try_into().unwrap();
            
            let mut hasher = ::sha2::Sha512::default();
            hasher.update($bytes);
            let buffer: [u8; 64] = hasher.finalize().as_slice().try_into().unwrap();

            use ark_ff::BigInteger;
            use ark_ff::FpParameters;
            use ark_ff::PrimeField;
            let MODULUS = ark_bls12_381::FrParameters::MODULUS;
            let mut b = ::ark_ff
                ::biginteger
                ::BigInteger256
                ::read(Cursor::new(buffer))
                .unwrap();
            while (b >= MODULUS) {
                b.sub_noborrow(&MODULUS);
            }

            // Panics if b is not normalized as above!
            let factor = ::ark_bls12_381::Fr::from_repr(b).unwrap();
            ::ark_bls12_381::G1Affine::from(
                ::ark_bls12_381::G1Affine::prime_subgroup_generator().mul(factor)
            )
        }
    }
}
