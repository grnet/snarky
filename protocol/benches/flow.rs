use protocol::{SRS, Trapdoor, BatchProof, Phase, Verification};
use circuits::ConstraintSystem;
use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    for (m, n, l) in [
        (30, 20, 10),
        (300, 200, 100),
        (3000, 2000, 1000),
    ].iter() {
        let qap = ConstraintSystem::create_default(*m, *n, *l).unwrap();
        group.bench_function(
            format!("create SRS with m:{}, n:{}, l:{}", m, n, l),
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
        let qap = ConstraintSystem::create_default(*m, *n, *l).unwrap();
        let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Phase 1 SRS update with m:{}, n:{}, l:{}", m, n, l),
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
        let qap = ConstraintSystem::create_default(*m, *n, *l).unwrap();
        let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
        let mut batch = BatchProof::initiate();
        group.bench_function(
            format!("Phase 2 SRS update with m:{}, n:{}, l:{}", m, n, l),
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
        let qap = ConstraintSystem::create_default(*m, *n, *l).unwrap();
        let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
        let mut batch = BatchProof::initiate();
        protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
        protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
        group.bench_function(
            format!("Verify SRS with m:{}, n:{}, l:{}", m, n, l),
            |b| b.iter(|| protocol::verify_naive(&qap, &srs, &batch)),
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
            format!("Verify SRS with m:{}, n:{}, l:{}", m, n, l),
            |b| b.iter(|| {
                let qap = ConstraintSystem::create_default(*m, *n, *l).unwrap();
                let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
                protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
                protocol::update(&qap, &mut srs, &mut batch, Phase::TWO);
                protocol::verify_naive(&qap, &srs, &batch)
            }),
        );
    }
    group.finish();
}

fn bench_verify_comparison(c: &mut Criterion) {
    let NR_1 = 500;
    let NR_2 = NR_1;

    let mut group = c.benchmark_group("verify");
    let qap = ConstraintSystem::create_default(5, 4, 3).unwrap();
    let (mut srs, trp) = SRS::setup_with_random_trapdoor(&qap);
    let mut batch = BatchProof::initiate();

    // phase 1 updates
    let mut count = 0;
    while count < NR_1 {
        protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
        count += 1;
    }

    // phase 2 updates
    let mut count = 0;
    while count < NR_2 {
        protocol::update(&qap, &mut srs, &mut batch, Phase::ONE);
        count += 1;
    }

    group.bench_function(
        format!("Non-batched verification"),
        |b| b.iter(|| protocol::verify_naive(&qap, &srs, &batch)),
    );
    group.bench_function(
        format!("Batched verification"),
        |b| b.iter(|| protocol::verify_naive(&qap, &srs, &batch)),
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_setup,
    bench_update_phase_1,
    bench_update_phase_2,
    bench_verify,
    bench_flow,
    bench_verify_comparison
);
criterion_main!(benches);
