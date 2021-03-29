use snarky;

use snarky::flow::{setup, update, verify, QAP};

fn main() {
    let srs = setup();
    let qap = QAP {};
    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);
}
