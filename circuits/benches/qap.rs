use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};

use polynomials::Univariate;
use circuits::ConstraintSystem;
use backend::scalar;

fn bench_create_default(c: &mut Criterion) {
    let mut group = c.benchmark_group("create_default");
    for (m, n, l) in [
        (10, 10, 10),
        (100, 100, 100),
        (1000, 1000, 1000),
    ].iter() {
        group.bench_function(
            format!("Create default QAP with m:{}, n:{}, l:{}", m, n, l),
            |b| b.iter(|| {
                ConstraintSystem::create_default(*m, *n, *l);
            }),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_create_default,
);
criterion_main!(benches);
