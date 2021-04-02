use std::time::Instant;
use snarky::flow::{QAP, Trapdoor, setup, update, verify};

fn main() {

    let start = Instant::now();
    println!("--------------------------");

    let m = 50;
    let n = 40;
    let l = 30;

    let qap_start = Instant::now();
    let qap = QAP::create_default(m, n, l);
    println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", m, n, l, qap_start.elapsed());

    let srs_start = Instant::now();
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    println!("[+] Initialized SRS ({:.2?})", srs_start.elapsed());

    let srs = update(&qap, &srs);
    let res = verify(&qap, &srs);
    assert!(res);

    println!("--------------------------");
    println!("Time elaped: {:.2?}", start.elapsed());
}
