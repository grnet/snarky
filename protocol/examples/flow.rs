use std::time::Instant;
use circuits::QAP;
use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
use protocol;

fn main() {

    let m = util::parse_arg(1, "50", "m should be a positive integer");
    let n = util::parse_arg(2, "40", "n should be a positive integer");
    let l = util::parse_arg(3, "30", "l should be a positive integer");

    let nr_1 = util::parse_arg(4, "3", "phase 1 repeats should be a non-negative integer");
    let nr_2 = util::parse_arg(5, "2", "phase 2 repeats should be a non-negative integer");

    println!("--------------------------");
    let start = Instant::now();

    let qap = {
        let start = Instant::now();
        match QAP::create_default(m, n, l) {
            Ok(qap) => {
                println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", 
                    m,
                    n,
                    l,
                    start.elapsed()
                );
                qap
            },
            Err(err) => {
                println!("{}", err); std::process::exit(1);
            }
        }
    };

    let (mut srs, trp) = {
        let start = Instant::now();
        let (srs, trp) = SRS::setup_with_unit_trapdoor(&qap);
        println!("[+] Initialized SRS ({:.2?})", start.elapsed());
        (srs, trp)
    };

    let mut batch = BatchProof::initiate();

    // phase 1 updates
    let mut count = 0;
    loop {
        let start = Instant::now();
        protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
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
        protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
        println!("[+] Phase 2 SRS update ({:.2?})", start.elapsed());
        count += 1;
        if count == nr_2 {
            break;
        }
    }

    let res = {
        let start = Instant::now();
        // let res = protocol::verify_naive(&qap, &srs, &batch);
        let res = protocol::verify(&qap, &srs, &batch);
        println!("[+] {:?} ({:.2?})", res, start.elapsed());
        res
    };
    assert!(bool::from(res));

    let elapsed = start.elapsed();
    println!("--------------------------");
    println!("Time elapsed: {:.2?}", elapsed);
}
