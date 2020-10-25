use clueengine::{ClueEngine, Card};

fn main() {
    let engines = vec![
        ("empty_game", ClueEngine::new(6, None).unwrap()),
        ("simple_monty_hall", make_simple_monty_hall_engine()),
        ("sample_from_website_1", make_sample_from_website_example()),
        ("sample_from_website_2", make_sample_from_website_example2()),
        ("add_card_expect_no_extras_test", make_add_card_expect_no_extras_test()),
        ("simulation_trickycase1", make_simulation_trickycase1()),
        ("simulation_trickycase2", make_simulation_trickycase2()),
        ("simulation_clauses_point_to_card", make_simulation_clauses_point_to_card()),
    ];
    for engine in engines.iter() {
        let sim_data = engine.1.do_simulation();
        println!("{}: {} total (out of {})", engine.0, sim_data.0.get(&Card::ProfessorPlum).unwrap().iter().sum::<usize>(), sim_data.1);
        //for card in CardUtils::all_cards() {
        //    println!("{:?}: {:?}", card, sim_data.get(&card).unwrap());
        //}
    }
}

fn make_simple_monty_hall_engine() -> ClueEngine {
    let mut clue_engine = ClueEngine::new(6, None).unwrap();
    clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(5), None);
    return clue_engine;
}

fn make_sample_from_website_example() -> ClueEngine {
    return ClueEngine::load_from_string("63-QLU.3-ANQIHOLUMG.3-QLU-AMG-ANH-AOI.3QLU-AFECSNBTIHKOGRPMJD.3-QLU.3-QLU.3-QLU.").unwrap();
}

fn make_sample_from_website_example2() -> ClueEngine {
    return ClueEngine::load_from_string("54TNJS-AFECBIHKOLURQPMGD.4-ANSTOJ-FHP.4-ANSTOKJP.3-FNSHTJP-AO-AK.3-FNSHTJP.3-TNJS.").unwrap();
}

fn make_add_card_expect_no_extras_test() -> ClueEngine {
    return ClueEngine::load_from_string("63FJQ-ABCDEGHIKLMNOPRSTU.3T-CDFHIJKNOPQS.3-CDFHIJKMNOPQST.3NO-CDFHIJKMPQST.3K-CDFHIJNOPQT.3CD-FJNOQT.3-CDFJNOQT.").unwrap();
}

fn make_simulation_trickycase1() -> ClueEngine {
    return ClueEngine::load_from_string("36CDKLQR-ABEFGHIJMNOPSTU.6T-BCDFGIKLQRS.6BF-CDKLPQRT.3-BCDFKLQRT.").unwrap();
}

fn make_simulation_trickycase2() -> ClueEngine {
    return ClueEngine::load_from_string("45CPQRS-ABDEFGHIJKLMNOTU.5AGIMT-BCDEFHJKLNOPQRSU.4FK-ACDGIJLMPQRST-EO.4DL-ABCFGIJKMPQRST.3J-ACDFGHIKLMPQRST.").unwrap();
}

fn make_simulation_clauses_point_to_card() -> ClueEngine {
    // In this game player with index 1 has a bunch of clauses that include Professor Plum and other cards,
    // so it's most likely he has that card
    return ClueEngine::load_from_string("36-GM.6-GM-AHN-AIO-AJP.6-GM.3GM-HIJKLNOPQRSTU.").unwrap();
}