use backend::*;
use circuits::ConstraintSystem;
use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
use protocol;
use num_traits::identities::Zero;
use ark_ec::AffineCurve;

macro_rules! run_setup {
    ($qap: expr, $trapdoor:expr) => {
        {
            match $trapdoor {
                "given"  => SRS::setup(&$qap, Some(Trapdoor::from_u64(1, 2, 3, 4))),
                "random" => SRS::setup_with_random_trapdoor(&$qap),
                "unit"   => SRS::setup_with_unit_trapdoor(&$qap),
                _        => panic!("Trapdoor generation unspecified")
            }
        }
    }
}

macro_rules! run_updates {
    ($qap: expr, $srs: expr, $batch: expr, $phase: expr, $nr: expr) => {
        let mut count = 0;
        while count < $nr {
            protocol::update(&$qap, &mut $srs, &mut $batch, $phase);
            count += 1
        }
    }
}

macro_rules! corrupt {
    ($batch: expr, $comp: expr, $index: expr) => {
        match $comp {
            1 => {
                let val = &$batch.batch_1[$index][0].com.0;
                $batch.batch_1[$index][0].com.0 = add1!(val, genG1!());
            }
            2 => {
                let val = &$batch.batch_2[$index].com.0;
                $batch.batch_2[$index].com.0 = add1!(val, genG1!());
            }
            _ => panic!("Batch component may be either 1 or 2")
        }

    }
}

macro_rules! run_protocol {
    (
        $m: expr, $n: expr, $l: expr, $trapdoor: expr, 
        $nr_1: expr => $cor_1: expr, $nr_2: expr => $cor_2: expr
    ) => {
        {
            let qap = ConstraintSystem::create_default($m, $n, $l).unwrap();
            let (mut srs, trp) = run_setup!(qap, $trapdoor);

            let mut batch = BatchProof::initiate();
            run_updates!(qap, srs, batch, Phase::ONE, $nr_1);
            run_updates!(qap, srs, batch, Phase::TWO, $nr_2);

            for (j, corrupt) in [$cor_1, $cor_2].iter().enumerate() {
                let comp = j + 1;
                let len = if comp == 1 { $nr_1 } else { $nr_2 };
                match corrupt {
                    &"all"        => for i in 0..len     { corrupt!(batch, comp, i); },
                    &"almost all" => for i in 0..len - 1 { corrupt!(batch, comp, i); },
                    &"one"        => corrupt!(batch, comp, len - 1),
                    _             => { /* Leave untouched*/ }
                }
            }

            let res = protocol::verify(&qap, &srs, &batch);
            assert_eq!(
                protocol::verify_naive(&qap, &srs, &batch), 
                res
            );                                              // cross-check
            res
        }
    }
}

#[test]
fn test_success_without_updates() {
    let res = run_protocol!(5, 4, 3, "random", 
        0 => "ok", 
        0 => "ok"
    );
    assert_eq!(bool::from(res), true);
}

#[test]
fn test_success_with_given_trapdoor() {
    let res = run_protocol!(5, 4, 3, "given", 
        1 => "ok", 
        1 => "ok"
    );
    assert_eq!(bool::from(res), true);
}

#[test]
fn test_success_with_random_trapdoor() {
    let res = run_protocol!(5, 4, 3, "random", 
        1 => "ok", 
        1 => "ok"
    );
    assert_eq!(bool::from(res), true);
}

#[test]
fn test_success_with_unit_trapdoor() {
    let res = run_protocol!(5, 4, 3, "unit", 
        1 => "ok", 
        1 => "ok"
    );
    assert_eq!(bool::from(res), true);
}

// #[test]
// fn test_failure_edge_1() {
//     let res = run_protocol!(5, 4, 3, "unit", 
//         1 => "one", 
//         1 => "ok"
//     );
//     assert_eq!(bool::from(res), false);
// }
// 
// #[test]
// fn test_failure_edge_2() {
//     let res = run_protocol!(5, 4, 3, "unit", 
//         1 => "ok", 
//         1 => "one"
//     );
//     assert_eq!(bool::from(res), false);
// }
// 
// #[test]
// fn test_failure_edge_3() {
//     let res = run_protocol!(5, 4, 3, "unit", 
//         2 => "one", 
//         1 => "ok"
//     );
//     assert_eq!(bool::from(res), false);
// }
// 
// #[test]
// fn test_failure_edge_4() {
//     let res = run_protocol!(5, 4, 3, "unit", 
//         1 => "ok", 
//         2 => "one"
//     );
//     assert_eq!(bool::from(res), false);
// }

#[test]
fn test_failure_with_one_phase_1_proof_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 
        2 => "one", 
        1 => "ok"
    );
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_failure_with_all_phase_1_proofs_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 
        2 => "all", 
        1 => "ok"
    );
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_failure_with_one_phase_2_proof_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 
        1 => "ok", 
        2 => "one"
    );
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_failure_with_all_phase_2_proofs_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 
        1 => "ok", 
        2 => "all"
    );
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_failure_with_all_but_one_phase_1_proof_tampered() {
    let res = run_protocol!(5, 4, 3, "unit", 
        5 => "almost all", 
        5 => "all"
    );
    assert_eq!(bool::from(res), false);
}

#[test]
fn test_failure_with_all_but_one_phase_2_proof_tampered() {
    let res = run_protocol!(5, 4, 3, "unit",
        5 => "all", 
        5 => "almost all"
    );
    assert_eq!(bool::from(res), false);
}

// Note: this test need not necessarily succeed, since it is below 
// minimum honesty assumptions; however, it generally does.
//
// #[test]
// fn test_failure_with_all_proofs_tampered() {
//     let res = run_protocol!(5, 4, 3, "unit", 
//         5 => "all", 
//         5 => "all"
//     );
//     assert_eq!(bool::from(res), false);
// }
