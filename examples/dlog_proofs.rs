use snarky::dlog::{prove_dlog, verify_dlog};
use snarky::{scalar, G1_gen, G2_gen, mult_1, mult_2};

pub fn main() {
    let elem_1 = mult_1!(G1_gen!(), scalar!(100));
    let elem_2 = mult_2!(G2_gen!(), scalar!(100));
    let phi = (elem_1, elem_2);
    let witness = scalar!(100);
    let proof = prove_dlog(phi, witness);
    let verified = verify_dlog(phi, proof);
    assert!(verified);
}
