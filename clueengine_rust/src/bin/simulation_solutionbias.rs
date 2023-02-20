use std::collections::HashMap;

use clueengine::{ClueEngine, Card};

fn main() {
    let engine = ClueEngine::load_from_string("36-.6--AHT.6-.3-.").unwrap();
    let old_simulation = engine.do_simulation(false);
    print_stats(&old_simulation, "Old");
    let new_simulation = engine.do_simulation(true);
    print_stats(&new_simulation, "New");
}

fn print_stats(simulation: &(HashMap<Card, Vec<usize>>, i32), description: &str) {
    let num_trials = simulation.0.get(&Card::Lounge).unwrap().iter().sum::<usize>();
    let percentage: f32 = *(simulation.0.get(&Card::Lounge).unwrap().last().unwrap()) as f32 / num_trials as f32;
    println!("{}: prob Lounge soln {:.3} ({} trials)", description, percentage, num_trials);
}