//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use snarky::flow::{QAP, Trapdoor, setup, update, verify};
mod common;

#[test]
fn test_flow() {
    let qap = QAP {};
    let trapdoor = Trapdoor {};
    let srs = setup(&trapdoor, &qap);
    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);
}
