use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};
use circuits::QAP;
use protocol::flow::{Trapdoor, Phase, BatchProof, setup, update, verify};

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trp = Trapdoor::create_from_units();
        group.bench_function(
            format!("create SRS with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| setup(&trp, &qap)),
        );
    }
    group.finish();
}

fn bench_update_phase_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trp = Trapdoor::create_from_units();
        let srs = setup(&trp, &qap);
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Phase 1 SRS update with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| update(&qap, &srs, &mut batch, Phase::ONE)),
        );
    }
    group.finish();
}

fn bench_update_phase_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trp = Trapdoor::create_from_units();
        let srs = setup(&trp, &qap);
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Phase 2 SRS update with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| update(&qap, &srs, &mut batch, Phase::TWO)),
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
        (1000, 1000, 1000),
    ].iter() {
        let qap = QAP::create_default(*m, *n, *l).unwrap();
        let trp = Trapdoor::create_from_units();
        let srs = setup(&trp, &qap);
        let mut batch = BatchProof::initiate();
        let srs = update(&qap, &srs, &mut batch, Phase::ONE);
        let srs = update(&qap, &srs, &mut batch, Phase::TWO);
        group.bench_function(
            format!("Verify SRS with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| verify(&qap, &srs, &batch)),
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
        (1000, 1000, 1000),
    ].iter() {
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Verify SRS with m:{}, n:{}, l{}", m, n, l),
            |b| b.iter(|| {
                let qap = QAP::create_default(*m, *n, *l).unwrap();
                let trp = Trapdoor::create_from_units();
                let srs = setup(&trp, &qap);
                let srs = update(&qap, &srs, &mut batch, Phase::ONE);
                let srs = update(&qap, &srs, &mut batch, Phase::TWO);
                verify(&qap, &srs, &batch)
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
