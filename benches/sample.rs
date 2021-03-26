// Note: Criterion only supports benchmarking of package level public functions

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

fn bench_func(c: &mut Criterion) {
    let mut group = c.benchmark_group("func");
    group.finish();
}

criterion_group!(
    benches,
    bench_func,
);
criterion_main!(benches);
