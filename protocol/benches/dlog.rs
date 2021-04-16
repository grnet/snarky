// Note: Criterion only supports benchmarking of package level public functions

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};
use protocol::dlog::{rndoracle, prove_dlog, verify_dlog};
use backend::{scalar, G1_gen, G2_gen, mult_1, mult_2};

fn bench_rndoracle(c: &mut Criterion) {
    let elem_1 = G1_gen!();
    let elem_2 = G2_gen!();
    let phi = (elem_1, elem_2);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| rndoracle(phi))
    );
}

fn bench_prove_dlog(c: &mut Criterion) {
    let elem_1 = mult_1!(G1_gen!(), scalar!(100));
    let elem_2 = mult_2!(G2_gen!(), scalar!(100));
    let phi = (elem_1, elem_2);
    let witness = scalar!(100);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| prove_dlog(phi, witness))
    );
}

fn bench_verify_dlog(c: &mut Criterion) {
    let G = G1_gen!();
    let H = G2_gen!();
    let elem_1 = mult_1!(G1_gen!(), scalar!(100));
    let elem_2 = mult_2!(G2_gen!(), scalar!(100));
    let phi = (elem_1, elem_2);
    let witness = scalar!(100);
    let proof = prove_dlog(phi, witness);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| verify_dlog(&G, &H, phi, proof))
    );
}

criterion_group!(
    benches,
    bench_rndoracle,
    bench_prove_dlog,
    bench_verify_dlog,
);
criterion_main!(benches);
