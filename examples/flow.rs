use std::time::Instant;
use snarky::flow::{QAP, Trapdoor, setup, update, verify};

fn parse_arg(pos: usize, default: &str, message: &str) -> usize {
    std::env::args()
        .nth(pos)
        .unwrap_or(default.to_string())
        .parse::<usize>()
        .ok()
        .expect(message)
}

fn main() {

    let m = parse_arg(1, "50", "m should be a positive integer");
    let n = parse_arg(2, "40", "n should be a positive integer");
    let l = parse_arg(3, "30", "l should be a positive integer");

    let start = Instant::now();
    println!("--------------------------");

    let qap_start = Instant::now();
    let qap = match QAP::create_default(m, n, l) {
        Ok(qap) => qap,
        Err(e)  => {
            println!("{}", e); std::process::exit(1);
        }
    };
    println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", m, n, l, qap_start.elapsed());

    let srs_start = Instant::now();
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    println!("[+] Initialized SRS ({:.2?})", srs_start.elapsed());

    // let srs = update(&qap, &srs);

    let ver_start = Instant::now();
    let res = verify(&qap, &srs);
    assert!(res);
    println!("[+] Verified SRS ({:.2?})", ver_start.elapsed());

    println!("--------------------------");
    println!("Time elaped: {:.2?}", start.elapsed());
}
