//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use snarky::flow::{QAP, Trapdoor, setup, update, verify};
mod common;

#[test]
fn test_flow() {
    let l = 5;
    let m = 4;
    let n = 3;
    let qap = QAP::create_default(l, m, n);
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    // let srs = update(&qap, &srs);    // TODO: Enable
    let res = verify(&qap, &srs);
    assert!(res);
}
