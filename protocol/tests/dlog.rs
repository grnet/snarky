use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::prover::{prove_dlog, verify_dlog};
use protocol::prover::ProofError;
use util::map;

#[test]
fn test_dlog_proof() {
    let parametrization = map! {
        (100, 100, 100) => true,
        (666, 100, 100) => false,
        (100, 666, 100) => false,
        (100, 100, 666) => false
    };
    for ((f1, f2, w), expected) in parametrization {
        let G = genG1!();
        let H = genG2!();
        let elem_1 = smul1!(scalar!(f1), genG1!());
        let elem_2 = smul2!(scalar!(f2), genG2!());
        let phi = (elem_1, elem_2);
        let witness = scalar!(w);
        let proof = prove_dlog(phi, witness);
        match expected {
            true => {
                assert!(verify_dlog(&G, &H, phi, proof).unwrap());
            },
            false => {
                assert_eq!(
                    verify_dlog(&G, &H, phi, proof).unwrap_err(), 
                    ProofError::DlogFailure
                );
            }
        };
    }
}
