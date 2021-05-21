use backend::{scalar, genG1, genG2, smul1, smul2};
use protocol::prover::Dlog;
use ark_ec::AffineCurve;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};

fn bench_rndoracle(c: &mut Criterion) {
    let elm1 = genG1!();
    let elm2 = genG2!();
    let commit = (elm1, elm2);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| Dlog::rndoracle(&commit))
    );
}

fn bench_prove_dlog(c: &mut Criterion) {
    let elm1 = smul1!(scalar!(100_u64), genG1!());
    let elm2 = smul2!(scalar!(100_u64), genG2!());
    let commit = (elm1, elm2);
    let witness = scalar!(100_u64);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| Dlog::prove(&commit, witness))
    );
}

fn bench_verify_dlog(c: &mut Criterion) {
    let ctx = (&genG1!(), &genG2!());
    let elm1 = smul1!(scalar!(100_u64), genG1!());
    let elm2 = smul2!(scalar!(100_u64), genG2!());
    let commit = (elm1, elm2);
    let witness = scalar!(100_u64);
    let proof = Dlog::prove(&commit, witness);
    c.bench_function(
        "scalar!",
        |b| b.iter(|| Dlog::verify(ctx, &commit, &proof))
    );
}

criterion_group!(
    benches,
    bench_rndoracle,
    bench_prove_dlog,
    bench_verify_dlog,
);
criterion_main!(benches);
