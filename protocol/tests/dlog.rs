use backend::{scalar, G1_gen, G2_gen, mult_1, mult_2};
use protocol::dlog::{prove_dlog, verify_dlog};
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
