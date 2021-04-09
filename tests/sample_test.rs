//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use snarky::QAP;
use snarky::flow::{Trapdoor, Phase, setup, update, verify};
mod common;

#[test]
fn test_flow() {
    let l = 5;
    let m = 4;
    let n = 3;
    let qap = QAP::create_default(l, m, n).unwrap();
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    let _srs = update(&qap, &srs, Phase::ONE);    // TODO: Enable
    let _srs = update(&qap, &srs, Phase::TWO);    // TODO: Enable
    let res = verify(&qap, &srs);
    assert!(res.as_bool());
}
