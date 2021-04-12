//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use snarky::QAP;
use snarky::flow::{Trapdoor, Phase, BatchProof, setup, update, verify};
use rand::RngCore;                  // Must be present for update
mod common;

#[test]
fn test_flow() {
    let l = 5;
    let m = 4;
    let n = 3;
    let qap = QAP::create_default(l, m, n).unwrap();
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    let mut batch = BatchProof::initiate();
    let mut rng = rand::thread_rng();
    let _srs = update(&qap, &srs, &mut batch, Phase::ONE, &mut rng);    // TODO: Enable
    let _srs = update(&qap, &srs, &mut batch, Phase::TWO, &mut rng);    // TODO: Enable
    let res = verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}
