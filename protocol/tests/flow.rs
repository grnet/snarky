//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use circuits::QAP;
use protocol::flow::{Trapdoor, SRS, Phase, BatchProof, update, verify};

#[test]
fn test_flow_with_given_trapdoor() {
    let (l, m, n) = (5, 4, 3);
    let qap = QAP::create_default(l, m, n).unwrap();
    let trp = Trapdoor::from_u64(1, 2, 3, 4);
    let (mut srs, trp) = SRS::setup(&qap, Some(trp));
    assert_eq!(trp, Trapdoor::from_u64(1, 2, 3, 4));
    let mut batch = BatchProof::initiate();
    update(&qap, &mut srs, &mut batch, Phase::ONE);
    update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}

#[test]
fn test_flow_with_random_trapdoor() {
    let (l, m, n) = (5, 4, 3);
    let qap = QAP::create_default(l, m, n).unwrap();
    let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
    assert_ne!(trp, Trapdoor::from_units());
    let mut batch = BatchProof::initiate();
    update(&qap, &mut srs, &mut batch, Phase::ONE);
    update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}

#[test]
fn test_flow_with_unit_trapdoor() {
    let (l, m, n) = (5, 4, 3);
    let qap = QAP::create_default(l, m, n).unwrap();
    let (mut srs, trp) = SRS::setup_with_unit_trapdoor(&qap);
    assert_eq!(trp, Trapdoor::from_units());
    let mut batch = BatchProof::initiate();
    update(&qap, &mut srs, &mut batch, Phase::ONE);
    update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}
