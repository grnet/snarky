use std::time::Instant;
use circuits::QAP;
use snarky::flow::{Trapdoor, Phase, BatchProof, setup, update, verify};

fn main() {

    let m = util::parse_arg(1, "50", "m should be a positive integer");
    let n = util::parse_arg(2, "40", "n should be a positive integer");
    let l = util::parse_arg(3, "30", "l should be a positive integer");

    let nr_1 = util::parse_arg(4, "3", "phase 1 repeats should be a non-negative integer");
    let nr_2 = util::parse_arg(5, "2", "phase 2 repeats should be a non-negative integer");

    use rand::RngCore;                  // Must be present for update
    let mut rng = rand::thread_rng();

    println!("--------------------------");
    let start = Instant::now();

    let qap_start = Instant::now();
    let qap = QAP::create_default(m, n, l)
        .unwrap_or_else(|err| {
            println!("{}", err); std::process::exit(1);
        });
    println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", m, n, l, qap_start.elapsed());

    let srs_start = Instant::now();
    let trapdoor = Trapdoor::create_from_units();
    let mut srs = setup(&trapdoor, &qap);
    println!("[+] Initialized SRS ({:.2?})", srs_start.elapsed());

    let mut batch = BatchProof::initiate();

    // phase 1 updates
    let mut count = 0;
    loop {
        let start = Instant::now();
        srs = update(&qap, &srs, &mut batch, Phase::ONE, &mut rng);
        println!("[+] Phase 1 SRS update ({:.2?})", start.elapsed());
        count += 1;
        if count == nr_1 {
            break;
        }
    }

    // phase 2 updates
    let mut count = 0;
    loop {
        let start = Instant::now();
        srs = update(&qap, &srs, &mut batch, Phase::TWO, &mut rng);
        println!("[+] Phase 2 SRS update ({:.2?})", start.elapsed());
        count += 1;
        if count == nr_2 {
            break;
        }
    }

    let ver_start = Instant::now();
    let res = verify(&qap, &srs, &batch);
    assert!(res.as_bool());
    println!("[+] {:?} ({:.2?})", res, ver_start.elapsed());

    let elapsed = start.elapsed();
    println!("--------------------------");
    println!("Time elaped: {:.2?}", elapsed);
}
