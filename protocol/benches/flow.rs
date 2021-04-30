use ark_ec::AffineCurve;            // Needed for group inclusion check
use ark_ec::PairingEngine;          // Needed for pairing
use num_traits::identities::Zero;   // Needed for zero constructions
use num_traits::identities::One;    // Needed for one constructions
use ark_ff::fields::Field;          // Needed for pow
use ark_ff::ToBytes;
use ark_std::rand::Rng as ArkRng;   // Must be in scope for rscalar
use ark_std::rand::RngCore;
use ark_bls12_381;

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};
use circuits::QAP;
use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
use protocol;

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    for (m, n, l) in [
        (30, 20, 10),
        (300, 200, 100),
        (3000, 2000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        group.bench_function(
            format!("create SRS with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| SRS::setup_with_random_trapdoor(&qap)),
        );
    }
    group.finish();
}

fn bench_update_phase_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (m, n, l) in [
        (30, 20, 10),
        (300, 200, 100),
        (3000, 2000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Phase 1 SRS update with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| protocol::update(&qap, &mut srs, &mut batch, Phase::ONE)),
        );
    }
    group.finish();
}

fn bench_update_phase_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (m, n, l) in [
        (30, 20, 10),
        (300, 200, 100),
        (3000, 2000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Phase 2 SRS update with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| protocol::update(&qap, &mut srs, &mut batch, Phase::TWO)),
        );
    }
    group.finish();
}

fn bench_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify");
    // TODO: Parametrize phases
    for (m, n, l) in [
        (30, 20, 10),
        (300, 200, 100),
        (3000, 2000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
        let mut batch = BatchProof::initiate();
        protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
        protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
        group.bench_function(
            format!("Verify SRS with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| protocol::verify(&qap, &srs, &batch)),
        );
    }
    group.finish();
}

fn bench_flow(c: &mut Criterion) {
    let mut group = c.benchmark_group("flow");
    // TODO: Parametrize phases
    for (m, n, l) in [
        (30, 20, 10),
        (300, 200, 100),
        (3000, 2000, 1000),
    ].iter() {
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Verify SRS with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| {
                let qap = QAP::create_default(*m, *n, *l).unwrap();
                let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
                protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
                protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
                protocol::verify(&qap, &srs, &batch)
            }),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_setup,
    bench_update_phase_1,
    bench_update_phase_2,
    bench_verify,
    bench_flow,
);
criterion_main!(benches);
