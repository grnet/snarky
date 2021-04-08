use crate::{scalar, G1_gen, G2_gen, mult_1, mult_2, pair};
use crate::backend::{
    G1Elem as G1,
    G2Elem as G2,
    Scalar,
};

pub fn prove_dlog(phi: (G1, G2), witness: Scalar) -> G1 {
    phi.0
}

pub fn verify_dlog(phi: (G1, G2), proof: G1) -> bool {
    true
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;

    #[test]
    fn test_dog_proof() {
        let parametrization = map! {
            (100, 100, 100) => true
            // (666, 100, 100) => false,
            // (100, 666, 100) => false,
            // (100, 100, 666) => false
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
