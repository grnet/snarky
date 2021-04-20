//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use circuits::QAP;
use protocol::flow::{Trapdoor, Phase, BatchProof, setup, update, verify};

#[test]
fn test_flow() {
    let l = 5;
    let m = 4;
    let n = 3;
    let qap = QAP::create_default(l, m, n).unwrap();
    let trp = Trapdoor::from_units();
    let mut srs = setup(&trp, &qap);
    let mut batch = BatchProof::initiate();
    update(&qap, &mut srs, &mut batch, Phase::ONE);
    update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}
