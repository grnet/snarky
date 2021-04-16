
use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

use backend::{
    scalar, zero, one, rscalar, G1_gen, G2_gen, G1_zero, G2_zero,
};

fn bench_scalar(c: &mut Criterion) {
    c.bench_function(
        "scalar!",
        |b| b.iter(|| scalar!(1000))
    );
}

fn bench_zero(c: &mut Criterion) {
    c.bench_function(
        "zero!",
        |b| b.iter(|| zero!())
    );
}

fn bench_one(c: &mut Criterion) {
    c.bench_function(
        "one!",
        |b| b.iter(|| one!())
    );
}

fn bench_rscalar(c: &mut Criterion) {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    c.bench_function(
        "rscalar!",
        |b| b.iter(|| rscalar!(rng))
    );
}

fn bench_G1_gen(c: &mut Criterion) {
    c.bench_function(
        "G1_gen!",
        |b| b.iter(|| G1_gen!())
    );
}

fn bench_G2_gen(c: &mut Criterion) {
    c.bench_function(
        "G2_gen!",
        |b| b.iter(|| G2_gen!())
    );
}

fn bench_G1_zero(c: &mut Criterion) {
    c.bench_function(
        "G1_zero!",
        |b| b.iter(|| G1_zero!())
    );
}

fn bench_G2_zero(c: &mut Criterion) {
    c.bench_function(
        "G2_zero!",
        |b| b.iter(|| G2_zero!())
    );
}

criterion_group!(
    benches,
    bench_scalar,
    bench_zero,
    bench_one,
    bench_rscalar,
    bench_G1_zero,
    bench_G2_zero,
    bench_G1_gen,
    bench_G2_gen,
);
criterion_main!(benches);
