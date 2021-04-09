// Note: Criterion only supports benchmarking of package level public functions

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};
use snarky::QAP;
use snarky::flow::{Trapdoor, Phase, setup, update, verify};

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        group.bench_function(
            format!("Generate SRS with l:{}, m:{}, n:{}", m, n, l),
            |b| b.iter(|| setup(&trapdoor, &qap)),
        );
    }
    group.finish();
}

fn bench_update_phase_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        let srs = setup(&trapdoor, &qap);
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        group.bench_function(
            format!("Phase 1 SRS update with l:{}, m:{}, n:{}", m, n, l),
            |b| b.iter(|| update(&qap, &srs, Phase::ONE, &mut rng)),
        );
    }
    group.finish();
}

fn bench_update_phase_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        let srs = setup(&trapdoor, &qap);
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        group.bench_function(
            format!("Phase 2 SRS update with l:{}, m:{}, n:{}", m, n, l),
            |b| b.iter(|| update(&qap, &srs, Phase::TWO, &mut rng)),
        );
    }
    group.finish();
}

fn bench_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify");
    // TODO: Parametrize phases
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        let srs = setup(&trapdoor, &qap);
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let srs = update(&qap, &srs, Phase::ONE, &mut rng);
        let srs = update(&qap, &srs, Phase::ONE, &mut rng);
        group.bench_function(
            format!("Verify SRS with l:{}, m:{}, n:{}", m, n, l),
            |b| b.iter(|| verify(&qap, &srs)),
        );
    }
    group.finish();
}

fn bench_flow(c: &mut Criterion) {
    let mut group = c.benchmark_group("flow");
    // TODO: Parametrize phases
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        group.bench_function(
            format!("Verify SRS with l:{}, m:{}, n:{}", m, n, l),
            |b| b.iter(|| {
                let qap = QAP::create_default(*m, *n, *l).unwrap();
                let trapdoor = Trapdoor::create_from_units();
                let srs = setup(&trapdoor, &qap);
                let srs = update(&qap, &srs, Phase::ONE, &mut rng);
                let srs = update(&qap, &srs, Phase::TWO, &mut rng);
                verify(&qap, &srs)
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
