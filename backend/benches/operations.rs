use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};

use backend::{scalar, genG1, genG2, zeroG1, zeroG2,
    pow, contained_in_group, add1, add2, smul1, smul2, pair,
    bytes1, bytes2, hashG1,
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
    let G = genG1!();
    let H = genG2!();
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

fn bench_add1(c: &mut Criterion) {
    let mut group = c.benchmark_group("add1");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let G = genG1!();
        let left  = smul1!(_f1, G);
        let right = smul1!(_f2, G);
        group.bench_function(
            format!("Compute {}G + {}G", f1, f2),
            |b| b.iter(|| add1!(left, right)),
        );
    }
    group.finish();
}

fn bench_add2(c: &mut Criterion) {
    let mut group = c.benchmark_group("add2");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let H = genG2!();
        let left  = smul2!(_f1, H);
        let right = smul2!(_f2, H);
        group.bench_function(
            format!("Compute {}H + {}H", f1, f2),
            |b| b.iter(|| add2!(left, right)),
        );
    }
    group.finish();
}

fn bench_smul1(c: &mut Criterion) {
    let mut group = c.benchmark_group("smul1");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let G = genG1!();
        let elm = smul1!(_f2, G);
        group.bench_function(
            format!("Compute {} * ({}G)", f1, f2),
            |b| b.iter(|| smul1!(_f1, elm)),
        );
    }
    group.finish();
}

fn bench_smul2(c: &mut Criterion) {
    let mut group = c.benchmark_group("smul2");
    for (f1, f2) in [
        (1, 10), (10, 100), (100, 1000), (1000, 10000),
    ].iter() {
        let _f1 = scalar!(*f1 as u64);
        let _f2 = scalar!(*f2 as u64);
        let H = genG2!();
        let elm = smul2!(_f2, H);
        group.bench_function(
            format!("Compute {} * ({}H)", f1, f2),
            |b| b.iter(|| smul2!(_f1, elm)),
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
        let G = genG1!();
        let H = genG2!();
        let left  = smul1!(_f1, G);
        let right = smul2!(_f2, H);
        group.bench_function(
            format!("Compute {}G o {}H", f1, f2),
            |b| b.iter(|| pair!(left, right)),
        );
    }
    group.finish();
}

fn bench_bytes1(c: &mut Criterion) {
    let zero = zeroG1!();
    c.bench_function(
        "bytes1!",
        |b| b.iter(|| bytes1!(zero))
    );
}

fn bench_bytes2(c: &mut Criterion) {
    let zero = zeroG2!();
    c.bench_function(
        "bytes2!",
        |b| b.iter(|| bytes2!(zero))
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
    bench_add1,
    bench_add2,
    bench_smul1,
    bench_smul2,
    bench_pair,
    bench_bytes1,
    bench_bytes2,
    bench_hashG1,
);
criterion_main!(benches);
