use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

use backend::{scalar, G1_gen, G2_gen, G1_zero, G2_zero,
    pow, contained_in_group, add_1, add_2, mult_1, mult_2, pair, 
    bytes_1, bytes_2, hashG1,
};


fn bench_power(c: &mut Criterion) {
    let base = scalar!(666);
    let exp = 999_usize;
    c.bench_function(
        "power!",
        |b| b.iter(|| pow!(base, exp))
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
    bench_power,
    bench_contained_in_group,
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
