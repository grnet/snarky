use crate::{scalar, G1_gen, G2_gen, mult_1, mult_2, pair, contained_in_group};
use crate::backend::{
    G1Elem as G1,
    G2Elem as G2,
    Scalar,
};

use sha2::Digest;   // Must be in scope for Sha256/512
use std::convert::TryInto;

pub fn hashG1(bytes: &[u8]) -> G1 {
    let mut hasher = ::sha2::Sha256::default();
    hasher.update(bytes);
    let buffer: [u8; 32] = hasher.finalize().try_into().unwrap();
    let factor = ::bls12_381::Scalar::from_bytes(&buffer).unwrap();
    ::bls12_381::G1Affine::from(::bls12_381::G1Affine::generator() * factor)
}

pub fn random_oracle(phi: (G1, G2)) -> G1 {
    hashG1(&[phi.0.to_uncompressed(), phi.1.to_compressed()].concat())
}

pub fn prove_dlog(phi: (G1, G2), witness: Scalar) -> G1 {
    mult_1!(random_oracle(phi), witness)
}

pub fn verify_dlog(phi: (G1, G2), proof: G1) -> bool {
    let G = G1_gen!();
    let H = G2_gen!();
    pair!(phi.0, H) == pair!(G, phi.1) && 
    pair!(proof, H) == pair!(random_oracle(phi), phi.1)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;

    #[test]
    fn test_dog_proof() {
        let parametrization = map! {
            (100, 100, 100) => true,
            (666, 100, 100) => false,
            (100, 666, 100) => false,
            (100, 100, 666) => false
        };
        for ((f1, f2, w), expected) in parametrization {
            let elem_1 = mult_1!(G1_gen!(), scalar!(f1));
            let elem_2 = mult_2!(G2_gen!(), scalar!(f2));
            let phi = (elem_1, elem_2);
            let witness = scalar!(w);
            let proof = prove_dlog(phi, witness);
            let verified = verify_dlog(phi, proof);
            assert_eq!(verified, expected);
        }
    }
}
