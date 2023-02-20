use std::time::Instant;

use clueengine::ClueEngine;

fn simulate_single() {
    // This is from add_card_expect_no_extras_test
    let clue_engine = ClueEngine::load_from_string("63FJQ-ABCDEGHIKLMNOPRSTU.3T-CDFHIJKNOPQS.3-CDFHIJKMNOPQST.3NO-CDFHIJKMPQST.3K-CDFHIJNOPQT.3CD-FJNOQT.3-CDFJNOQT.").unwrap();
    clue_engine.do_simulation(false);
}

fn main() {
    let start = Instant::now();
    for _ in 0..10 {
        simulate_single();
    }
    println!("elapsed - {:?}", start.elapsed());
}