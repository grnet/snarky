use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};

use polynomials::Univariate;
use backend::scalar;

fn bench_evaluate(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluate");
    let x = scalar!(666);
    for exp in [0, 1, 2, 3, 4, 5, 6].iter() {
       let deg = 10_u64.pow(*exp);
       let poly = Univariate::create_from_u64(&(0..deg + 1).collect());
        group.bench_function(
            format!("Polynomial of degree 10 ^ {}", *exp),
            |b| b.iter(|| poly.evaluate(&x)),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_evaluate,
);
criterion_main!(benches);
