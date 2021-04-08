// Note: Criterion only supports benchmarking of package level public functions

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};
use snarky::flow::{QAP, Trapdoor, setup, update, verify};

fn bench_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    for (l, m, n) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*l, *m, *n).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        group.bench_function(
            format!("Generate SRS with l:{}, m:{}, n:{}", l, m, n),
            |b| b.iter(|| setup(&trapdoor, &qap)),
        );
    }
    group.finish();
}

fn bench_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    for (l, m, n) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*l, *m, *n).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        let srs = setup(&trapdoor, &qap);
        group.bench_function(
            format!("Update SRS with l:{}, m:{}, n:{}", l, m, n),
            |b| b.iter(|| update(&qap, &srs)),
        );
    }
    group.finish();
}

fn bench_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify");
    for (l, m, n) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        let qap = QAP::create_default(*l, *m, *n).unwrap();
        let trapdoor = Trapdoor::create_from_units();
        let srs = setup(&trapdoor, &qap);
        let srs = update(&qap, &srs);
        group.bench_function(
            format!("Verify SRS with l:{}, m:{}, n:{}", l, m, n),
            |b| b.iter(|| verify(&qap, &srs)),
        );
    }
    group.finish();
}

fn bench_flow(c: &mut Criterion) {
    let mut group = c.benchmark_group("flow");
    // TODO: Parametrize with QAP dimensions instead
    for (l, m, n) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 100),
    ].iter() {
        group.bench_function(
            format!("Verify SRS with l:{}, m:{}, n:{}", l, m, n),
            |b| b.iter(|| {
                let qap = QAP::create_default(*l, *m, *n).unwrap();
                let trapdoor = Trapdoor::create_from_units();
                let srs = setup(&trapdoor, &qap);
                let srs = update(&qap, &srs);
                verify(&qap, &srs)
            }),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_setup,
    bench_update,
    bench_verify,
    bench_flow,
);
criterion_main!(benches);
