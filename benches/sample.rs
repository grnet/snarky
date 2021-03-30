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
    // TODO: Parametrize with QAP dimensions instead
    for size in [10, 100, 1000].iter() {
        let qap = QAP {};
        let trapdoor = Trapdoor {};
        group.bench_function(
            format!("Generate SRS with size {}", size),
            |b| b.iter(|| setup(&trapdoor, &qap)),
        );
    }
    group.finish();
}

fn bench_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("update");
    // TODO: Parametrize with QAP dimensions instead
    for size in [10, 100, 1000].iter() {
        let qap = QAP {};
        let trapdoor = Trapdoor {};
        let srs = setup(&trapdoor, &qap);
        group.bench_function(
            format!("Update SRS with size {}", size),
            |b| b.iter(|| update(&qap, &srs)),
        );
    }
    group.finish();
}

fn bench_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify");
    // TODO: Parametrize with QAP dimensions instead
    for size in [10, 100, 1000].iter() {
        let qap = QAP {};
        let trapdoor = Trapdoor {};
        let srs = setup(&trapdoor, &qap);
        let srs = update(&qap, &srs);
        group.bench_function(
            format!("Verify SRS with size {}", size),
            |b| b.iter(|| verify(&qap, &srs)),
        );
    }
    group.finish();
}

fn bench_flow(c: &mut Criterion) {
    let mut group = c.benchmark_group("flow");
    // TODO: Parametrize with QAP dimensions instead
    for size in [10, 100, 1000].iter() {
        group.bench_function(
            format!("Bench overall flow with size {}", size),
            |b| b.iter(|| {
                let qap = QAP {};
                let trapdoor = Trapdoor {};
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
