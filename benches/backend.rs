// Note: Criterion only supports benchmarking of package level public functions

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

use snarky::{
    scalar, zero, one, rndscalar, pow, G1_gen, G2_gen, contained_in_group, 
    add_1, add_2, G1_zero, G2_zero, mult_1, mult_2, pair, bytes_1, bytes_2, hashG1, 
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

fn bench_rndscalar(c: &mut Criterion) {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    c.bench_function(
        "rndscalar!",
        |b| b.iter(|| rndscalar!(rng))
    );
}

fn bench_power(c: &mut Criterion) {
    c.bench_function(
        "power!",
        |b| b.iter(|| one!())
    );
}

fn bench_G1_gen(c: &mut Criterion) {
    let base = scalar!(666);
    let exp = 999_usize;
    c.bench_function(
        "G1_gen!",
        |b| b.iter(|| pow!(base, exp))
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

fn bench_contained_in_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("contained_in_group");
    let G = G1_gen!();
    let H = G2_gen!();
    group.bench_function(
        format!("Chech G in G_1"),
        |b| b.iter(|| contained_in_group!(G)),
    );
    group.bench_function(
        format!("Chech H in G_2"),
        |b| b.iter(|| contained_in_group!(H)),
    );
    group.finish();
}

fn bench_add_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_1");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let G = G1_gen!();
        let left  = mult_1!(G, _f1);
        let right = mult_1!(G, _f2);
        group.bench_function(
            format!("Compute {}G + {}G", f1, f2),
            |b| b.iter(|| add_1!(left, right)),
        );
    }
    group.finish();
}

fn bench_add_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_2");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let H = G2_gen!();
        let left  = mult_2!(H, _f1);
        let right = mult_2!(H, _f2);
        group.bench_function(
            format!("Compute {}H + {}H", f1, f2),
            |b| b.iter(|| add_2!(left, right)),
        );
    }
    group.finish();
}

fn bench_mult_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("mult_1");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let G = G1_gen!();
        let elm = mult_1!(G, _f2);
        group.bench_function(
            format!("Compute {} * ({}G)", f1, f2),
            |b| b.iter(|| mult_1!(elm, _f1)),
        );
    }
    group.finish();
}

fn bench_mult_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("mult_2");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let H = G2_gen!();
        let elm = mult_2!(H, _f2);
        group.bench_function(
            format!("Compute {} * ({}H)", f1, f2),
            |b| b.iter(|| mult_2!(elm, _f1)),
        );
    }
    group.finish();
}

fn bench_pair(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    for (f1, f2) in [
        (1, 1), (10, 10), (100, 100), (1000, 1000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let G = G1_gen!();
        let H = G2_gen!();
        let left  = mult_1!(G, _f1);
        let right = mult_2!(H, _f2);
        group.bench_function(
            format!("Compute {}G o {}H", f1, f2),
            |b| b.iter(|| pair!(left, right)),
        );
    }
    group.finish();
}

fn bench_bytes_1(c: &mut Criterion) {
    let zero = G1_zero!();
    c.bench_function(
        "bytes_1!",
        |b| b.iter(|| bytes_1!(zero))
    );
}

fn bench_bytes_2(c: &mut Criterion) {
    let zero = G2_zero!();
    c.bench_function(
        "bytes_2!",
        |b| b.iter(|| bytes_2!(zero))
    );
}

fn bench_hashG1(c: &mut Criterion) {
    use sha2::Digest;
    use std::convert::TryInto;
    let bytes: Vec<u8> = (0..5).collect();
    c.bench_function(
        "hashG1!",
        |b| b.iter(|| hashG1!(&bytes))
    );
}



criterion_group!(
    benches,
    bench_scalar,
    bench_zero,
    bench_one,
    bench_rndscalar,
    bench_G1_zero,
    bench_G2_zero,
    bench_contained_in_group,
    bench_G1_gen,
    bench_G2_gen,
    bench_add_1,
    bench_add_2,
    bench_mult_1,
    bench_mult_2,
    bench_pair,
    bench_bytes_1,
    bench_bytes_2,
    bench_hashG1,
);
criterion_main!(benches);
