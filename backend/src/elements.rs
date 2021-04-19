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
macro_rules! pow {
    ($base:expr, $exp:expr) => {
        $base.pow(&[$exp as u64, 0, 0, 0])
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
macro_rules! contained_in_group {
    ($elem:expr) => {
        bool::from($elem.is_on_curve())
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
