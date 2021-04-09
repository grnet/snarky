use std::time::Instant;
use snarky::QAP;
use snarky::flow::{Trapdoor, Phase, setup, update, verify};

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

    use rand::RngCore;                  // Must be present for update
    let mut rng = rand::thread_rng();

    let start = Instant::now();
    println!("--------------------------");

    let qap_start = Instant::now();
    let qap = QAP::create_default(m, n, l)
        .unwrap_or_else(|err| {
            println!("{}", err); std::process::exit(1);
        });
    println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", m, n, l, qap_start.elapsed());

    let srs_start = Instant::now();
    let trapdoor = Trapdoor::create_from_units();
    let srs = setup(&trapdoor, &qap);
    println!("[+] Initialized SRS ({:.2?})", srs_start.elapsed());

    let phase1_start = Instant::now();
    let _srs = update(&qap, &srs, Phase::ONE, &mut rng);
    println!("[+] Phase 1 SRS update ({:.2?})", phase1_start.elapsed());

    let phase2_start = Instant::now();
    let _srs = update(&qap, &srs, Phase::TWO, &mut rng);
    println!("[+] Phase 2 SRS update ({:.2?})", phase2_start.elapsed());

    let ver_start = Instant::now();
    let res = verify(&qap, &srs);
    assert!(res.as_bool());
    println!("[+] {:?} ({:.2?})", res, ver_start.elapsed());

    println!("--------------------------");
    println!("Time elaped: {:.2?}", start.elapsed());
}
