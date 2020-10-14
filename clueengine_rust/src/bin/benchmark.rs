use std::time::Instant;
use clueengine::ClueEngine;

fn main() {
    let start = Instant::now();
    //let clue_engine = ClueEngine::load_from_string("45CPQRS-ABDEFGHIJKLMNOTU.5AGIMT-BCDEFHJKLNOPQRSU.4FK-ACDGIJLMPQRST-EO.4DL-ABCFGIJKMPQRST.3J-ACDFGHIKLMPQRST.").unwrap();
    let clue_engine = ClueEngine::load_from_string("63A-.3-A.3-A.3-A.3-A.3-A.3-A.").unwrap();

    for _ in 0..10 {
        let simulation_data = clue_engine.do_simulation();
        let (card, data) = simulation_data.iter().next().unwrap();
        // Print something to make sure nothing gets optimized out
        // But don't print everything, that makes the console the long pole
        println!("{:?}: {:?}", *card, *data);
        /*for (card, data) in simulation_data.iter() {
            println!("{:?}: {:?}", *card, *data);
        }*/
    }
    println!("{:?}", start.elapsed());
}