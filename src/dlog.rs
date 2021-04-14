use crate::{scalar, G1_gen, G2_gen, mult_1, mult_2, pair, contained_in_group, 
    bytes_1, bytes_2, hashG1};
use crate::backend::{
    G1Elem as G1,
    G2Elem as G2,
    Scalar,
};

use sha2::Digest;               // Must be in scope for hashG1
use std::convert::TryInto;      // Must be in scope for hashG1

pub fn rndoracle(phi: (G1, G2)) -> G1 {
    hashG1!(&[bytes_1!(phi.0), bytes_2!(phi.1)].concat())
}

pub fn prove_dlog(phi: (G1, G2), witness: Scalar) -> G1 {
    mult_1!(rndoracle(phi), witness)
}

pub fn verify_dlog(G: &G1, H: &G2, phi: (G1, G2), proof: G1) -> bool {
    pair!(phi.0, H) == pair!(G, phi.1) && 
    pair!(proof, H) == pair!(rndoracle(phi), phi.1)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;

    #[test]
    fn test_dlog_proof() {
        let parametrization = map! {
            (100, 100, 100) => true,
            (666, 100, 100) => false,
            (100, 666, 100) => false,
            (100, 100, 666) => false
        };
        for ((f1, f2, w), expected) in parametrization {
            let G = G1_gen!();
            let H = G2_gen!();
            let elem_1 = mult_1!(G1_gen!(), scalar!(f1));
            let elem_2 = mult_2!(G2_gen!(), scalar!(f2));
            let phi = (elem_1, elem_2);
            let witness = scalar!(w);
            let proof = prove_dlog(phi, witness);
            let verified = verify_dlog(&G, &H, phi, proof);
            assert_eq!(verified, expected);
        }
    }
}
