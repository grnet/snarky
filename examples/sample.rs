use snarky;

use snarky::flow::{QAP, Trapdoor, setup, update, verify};

fn main() {
    let qap = QAP {};
    let trapdoor = Trapdoor {};
    let srs = setup(&trapdoor, &qap);
    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);
}
