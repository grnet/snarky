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
}
