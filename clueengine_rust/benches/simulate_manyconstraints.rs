use criterion::{criterion_group, criterion_main, Criterion};
use clueengine::ClueEngine;

fn simulate_single() {
    let clue_engine = ClueEngine::load_from_string("63BHS-ACDEFGIJKLMNOPQRTU.3-BDHKQS.3-ABDHKQST.3-BDHKLPQS-AT.3Q-BHLPS.3-BHLPQS.3-BHQS.").unwrap();
    clue_engine.do_simulation(false);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simulate_manyconstraints", |b| b.iter(|| simulate_single()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);