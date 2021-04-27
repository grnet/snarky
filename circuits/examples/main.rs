use std::time::Instant;
use backend;
use circuits::{QAP, ExpQAP};

fn main() {

    let m = util::parse_arg(1, "50", "m should be a positive integer");
    let n = util::parse_arg(2, "40", "n should be a positive integer");
    let l = util::parse_arg(3, "30", "l should be a positive integer");

    println!("-------------------");
    let start = Instant::now();
    let qap = QAP::create_default(m, n, l)
        .unwrap_or_else(|err| {
            println!("{}", err); std::process::exit(1);
        });
    println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", m, n, l, start.elapsed());

    println!("-------------------");
    let start = Instant::now();
    let qap = ExpQAP::<backend::RcBls12_381>::create_default(m, n, l)
        .unwrap_or_else(|err| {
            println!("{}", err); std::process::exit(1);
        });
    println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", m, n, l, start.elapsed());
}
