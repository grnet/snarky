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
macro_rules! rscalar {
    ($rng:expr) => {
        {
            let mut buf = [0; 64];
            $rng.fill_bytes(&mut buf);
            ::bls12_381::Scalar::from_bytes_wide(&buf)
        }
    }
}

#[macro_export]
macro_rules! genG1 {
    () => {
        ::bls12_381::G1Affine::generator()
    }
}

#[macro_export]
macro_rules! genG2 {
    () => {
        ::bls12_381::G2Affine::generator()
    }
}

#[macro_export]
macro_rules! zeroG1 {
    () => {
        ::bls12_381::G1Affine::default();
    }
}

#[macro_export]
macro_rules! zeroG2 {
    () => {
        ::bls12_381::G2Affine::default();
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
        $elem.to_uncompressed()     // 96 bytes
    }
}

#[macro_export]
macro_rules! bytes2 {
    ($elem:expr) => {
        $elem.to_compressed()       // 96 bytes
    }
}

#[macro_export]
// Constant-time equality check
// Note: Applies to all types of elements for bls12_381. The
// bls12_831 backend uses subtle for contant-time operations:
// https://docs.rs/subtle/2.4.0/subtle/
macro_rules! ct_eq {
    ($elem1:expr, $elem2:expr) => {
        // https://docs.rs/subtle/2.4.0/src/subtle/lib.rs.html#67
        bool::from($elem1.ct_eq(&$elem2))
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
        !bool::from($elem1.ct_eq(&$elem2))
    }
}
