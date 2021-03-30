use std::time::Instant;
use snarky::flow::{QAP, Trapdoor, setup, update, verify};

fn main() {

    let start = Instant::now();
    println!("--------------------------");

    let l = 5;
    let m = 4;
    let n = 3;
    let qap = QAP::create_default(l, m, n);
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);

    println!("--------------------------");
    println!("Time elaped: {:.2?}", start.elapsed());
}
