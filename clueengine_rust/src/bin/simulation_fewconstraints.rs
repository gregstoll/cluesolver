use std::time::Instant;

use clueengine::ClueEngine;

fn simulate_single() {
    let clue_engine = ClueEngine::load_from_string("63A-.3-A.3-A.3-A.3-A.3-A.3-A.").unwrap();
    clue_engine.do_simulation();
}

fn main() {
    let start = Instant::now();
    for _ in 0..10 {
        simulate_single();
    }
    println!("elapsed - {:?}", start.elapsed());
}