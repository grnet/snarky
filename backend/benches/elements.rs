use ark_ec::AffineCurve;
use num_traits::identities::Zero;
use num_traits::identities::One;

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

use backend::{
    scalar, zero, one, rscalar, genG1, genG2, zeroG1, zeroG2,
};

fn bench_scalar(c: &mut Criterion) {
    let val = 1000u64;
    c.bench_function(
        "scalar!",
        |b| b.iter(|| scalar!(val))
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
    use ark_std::rand::Rng;
    use ark_std::rand::RngCore as ArkRngCore;
    use ark_std::rand::SeedableRng;
    c.bench_function(
        "rscalar!",
        |b| b.iter(|| rscalar!(::util::snarky_rng()))
    );
}

fn bench_genG1(c: &mut Criterion) {
    c.bench_function(
        "genG1!",
        |b| b.iter(|| genG1!())
    );
}

fn bench_genG2(c: &mut Criterion) {
    c.bench_function(
        "genG2!",
        |b| b.iter(|| genG2!())
    );
}

fn bench_zeroG1(c: &mut Criterion) {
    c.bench_function(
        "zeroG1!",
        |b| b.iter(|| zeroG1!())
    );
}

fn bench_zeroG2(c: &mut Criterion) {
    c.bench_function(
        "zeroG2!",
        |b| b.iter(|| zeroG2!())
    );
}

criterion_group!(
    benches,
    bench_scalar,
    bench_zero,
    bench_one,
    bench_rscalar,
    bench_zeroG1,
    bench_zeroG2,
    bench_genG1,
    bench_genG2,
);
criterion_main!(benches);
