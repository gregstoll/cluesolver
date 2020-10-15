use std::time::Instant;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use clueengine::ClueEngine;

/*fn main() {
    let start = Instant::now();
    let clue_engine = ClueEngine::load_from_string("63A-.3-A.3-A.3-A.3-A.3-A.3-A.").unwrap();

    for _ in 0..10 {
        let simulation_data = clue_engine.do_simulation();
        let (card, data) = simulation_data.iter().next().unwrap();
        // Print something to make sure nothing gets optimized out
        // But don't print everything, that makes the console the long pole
        println!("{:?}: {:?}", *card, *data);
    }
    println!("{:?}", start.elapsed());
}*/

fn simulate_single() {
    let clue_engine = ClueEngine::load_from_string("63A-.3-A.3-A.3-A.3-A.3-A.3-A.").unwrap();
    let simulation_data = clue_engine.do_simulation();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simulate_fewconstraints", |b| b.iter(|| simulate_single()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);