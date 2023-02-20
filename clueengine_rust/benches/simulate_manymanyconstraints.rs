use criterion::{criterion_group, criterion_main, Criterion};
use clueengine::ClueEngine;

fn simulate_single() {
    // This is from add_card_expect_no_extras_test
    let clue_engine = ClueEngine::load_from_string("63FJQ-ABCDEGHIKLMNOPRSTU.3T-CDFHIJKNOPQS.3-CDFHIJKMNOPQST.3NO-CDFHIJKMPQST.3K-CDFHIJNOPQT.3CD-FJNOQT.3-CDFJNOQT.").unwrap();
    clue_engine.do_simulation(false);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simulate_manymanyconstraints", |b| b.iter(|| simulate_single()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);