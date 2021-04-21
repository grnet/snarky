use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::prover::{Dlog, ProofError};
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
        let ctx = (&genG1!(), &genG2!());
        let elm1 = smul1!(scalar!(f1), genG1!());
        let elm2 = smul2!(scalar!(f2), genG2!());
        let commit = (elm1, elm2);
        let witness = scalar!(w);
        let proof = Dlog::prove(&commit, witness);
        match expected {
            true => {
                assert!(Dlog::verify(ctx, &commit, &proof).unwrap());
            },
            false => {
                assert_eq!(
                    Dlog::verify(ctx, &commit, &proof).unwrap_err(), 
                    ProofError::DlogFailure
                );
            }
        };
    }
}
