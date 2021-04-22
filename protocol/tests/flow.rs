//! Contains integration tests involving `snarky` public functions.
//! Common test setup taken from the `common` module.

use circuits::QAP;
use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
use protocol;

#[test]
fn test_flow_with_given_trapdoor() {
    let (m, n, l) = (5, 4, 3);
    let qap = QAP::create_default(m, n, l).unwrap();
    let trp = Trapdoor::from_u64(1, 2, 3, 4);
    let (mut srs, trp) = SRS::setup(&qap, Some(trp));
    assert_eq!(trp, Trapdoor::from_u64(1, 2, 3, 4));
    let mut batch = BatchProof::initiate();
    protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
    protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = protocol::verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}

#[test]
fn test_flow_with_random_trapdoor() {
    let (m, n, l) = (5, 4, 3);
    let qap = QAP::create_default(m, n, l).unwrap();
    let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
    assert_ne!(trp, Trapdoor::from_units());
    let mut batch = BatchProof::initiate();
    protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
    protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = protocol::verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}

#[test]
fn test_flow_with_unit_trapdoor() {
    let (m, n, l) = (5, 4, 3);
    let qap = QAP::create_default(m, n, l).unwrap();
    let (mut srs, trp) = SRS::setup_with_unit_trapdoor(&qap);
    assert_eq!(trp, Trapdoor::from_units());
    let mut batch = BatchProof::initiate();
    protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
    protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
    let res = protocol::verify(&qap, &srs, &batch);
    assert!(res.as_bool());
}
