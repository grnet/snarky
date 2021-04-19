// Note: Criterion only supports benchmarking of package level public functions

use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
    BenchmarkId,
};
use protocol::dlog::{rndoracle, prove_dlog, verify_dlog};
use backend::{scalar, genG1, genG2, smul1, smul2};

fn bench_rndoracle(c: &mut Criterion) {
    let elem_1 = genG1!();
    let elem_2 = genG2!();
    let phi = (elem_1, elem_2);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| rndoracle(phi))
    );
}

fn bench_prove_dlog(c: &mut Criterion) {
    let elem_1 = smul1!(genG1!(), scalar!(100));
    let elem_2 = smul2!(genG2!(), scalar!(100));
    let phi = (elem_1, elem_2);
    let witness = scalar!(100);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| prove_dlog(phi, witness))
    );
}

fn bench_verify_dlog(c: &mut Criterion) {
    let G = genG1!();
    let H = genG2!();
    let elem_1 = smul1!(genG1!(), scalar!(100));
    let elem_2 = smul2!(genG2!(), scalar!(100));
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
