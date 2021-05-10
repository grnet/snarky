// use std::time::Instant;
// use circuits::QAP;
// use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
// use protocol;
use clap::{Arg, ArgMatches, App, AppSettings, Clap};
use std::str::FromStr;

// fn parse_string<T: FromStr>(matches: &ArgMatches, arg_name: &str) -> T 
//     where T: FromStr<Err = ::std::fmt::Debug>
// {
//     matches
//         .value_of(arg_name)
//         .unwrap()
//         .to_string()
//         .parse::<T>()
// }

fn main() {

    #[derive(Clap)]
    #[clap(version = "1.0", author = "GRNET")]
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(about = "Simulates execution of the Snarky Ceremonies protocol")]
    struct Opts {

        #[clap(parse(try_from_str), default_value = "5", about = "some thing")]
        m: usize,

        #[clap(long, raw(false), parse(try_from_str), default_value = "false", about = "some other")]
        naive: bool,
    }

    let opts: Opts = Opts::parse();

    println!("Value for m: {}", opts.naive);
    println!("Value for naive: {}", opts.naive);

    // let matches = App::new("snarky demo script")
    //     .author("GRNET")
    //     .version("1.0")
    //     .about("Simulates execution of the Snarky Ceremonies protocol")
    //     .arg(Arg::new("MDIM")
    //         .about("m dimension of constraint system")
    //         .value_name("m")
    //         .default_value("5")
    //         .index(1))
    //     .arg(Arg::new("NDIM")
    //         .about("n dimension of constraint system")
    //         .value_name("n")
    //         .default_value("4")
    //         .index(2))
    //     .arg(Arg::new("LDIM")
    //         .about("l dimension of constraint system")
    //         .value_name("l")
    //         .default_value("3")
    //         .index(3))
    //     // .arg("--nr1=[NR1] 'Number of phase 1 updates'")
    //     // .arg(Arg::new("nr1")
    //     //     .about("Number of phase 1 updates")
    //     //     .long("nr1")
    //     //     .value_name("NR1")
    //     //     .default_value("5"))
    //     .subcommand(App::new("naive")
    //         .about("Performs non-batched verification"))
    //     .get_matches();

    // // let Some(m) = matches.value_of("MDIM")
    // let m = matches.value_of("MDIM").unwrap().to_string().parse::<usize>().unwrap();
    // let n = matches.value_of("NDIM").unwrap().to_string().parse::<usize>().unwrap();
    // let l = matches.value_of("LDIM").unwrap().to_string().parse::<usize>().unwrap();

    // println!("{:?}", m);
    // println!("{:?}", n);
    // println!("{:?}", l);


    // let Some(m) = matches.value_of("MDIM")
    // println!("{:?}", m);

    // let m = util::parse_arg(1, "50", "m should be a positive integer");
    // let n = util::parse_arg(2, "40", "n should be a positive integer");
    // let l = util::parse_arg(3, "30", "l should be a positive integer");

    // let nr_1 = util::parse_arg(4, "3", "phase 1 repeats should be a non-negative integer");
    // let nr_2 = util::parse_arg(5, "2", "phase 2 repeats should be a non-negative integer");

    // println!("--------------------------");
    // let start = Instant::now();

    // let qap = {
    //     let start = Instant::now();
    //     match QAP::create_default(m, n, l) {
    //         Ok(qap) => {
    //             println!("[+] Created QAP with m:{} n:{} l:{} ({:.2?})", 
    //                 m, 
    //                 n, 
    //                 l, 
    //                 start.elapsed()
    //             );
    //             qap
    //         },
    //         Err(err) => {
    //             println!("{}", err); std::process::exit(1);
    //         }
    //     }
    // };

    // let (mut srs, trp) = {
    //     let start = Instant::now();
    //     let (srs, trp) = SRS::setup_with_unit_trapdoor(&qap);
    //     println!("[+] Initialized SRS ({:.2?})", start.elapsed());
    //     (srs, trp)
    // };

    // let mut batch = BatchProof::initiate();

    // // phase 1 updates
    // let mut count = 0;
    // loop {
    //     let start = Instant::now();
    //     protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
    //     println!("[+] Phase 1 SRS update ({:.2?})", start.elapsed());
    //     count += 1;
    //     if count == nr_1 {
    //         break;
    //     }
    // }

    // // phase 2 updates
    // let mut count = 0;
    // loop {
    //     let start = Instant::now();
    //     protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
    //     println!("[+] Phase 2 SRS update ({:.2?})", start.elapsed());
    //     count += 1;
    //     if count == nr_2 {
    //         break;
    //     }
    // }

    // let res = {
    //     let start = Instant::now();
    //     let res = protocol::verify(&qap, &srs, &batch);
    //     println!("[+] {:?} ({:.2?})", res, start.elapsed());
    //     res
    // };
    // assert!(bool::from(res));

    // let elapsed = start.elapsed();
    // println!("--------------------------");
    // println!("Time elapsed: {:.2?}", elapsed);
}
