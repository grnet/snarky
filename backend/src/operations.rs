#[macro_export]
macro_rules! add1 {
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
macro_rules! add2 {
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
macro_rules! smul1 {
    ($factor: expr, $elem: expr) => {
        ::bls12_381::G1Affine::from($elem * $factor)
    }
}

#[macro_export]
macro_rules! smul2 {
    ($factor: expr, $elem: expr) => {
        ::bls12_381::G2Affine::from($elem * $factor)
    }
}

#[macro_export]
macro_rules! pair {
    ($left:expr, $right:expr) => {
        ::bls12_381::pairing(&$left, &$right)
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
