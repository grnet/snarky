//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use snarky::flow::{setup, update, verify, QAP};
mod common;

#[test]
fn test_flow() {
    let srs = setup();
    let qap = QAP {};
    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);
}
