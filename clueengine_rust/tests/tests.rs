#[cfg(test)]
mod tests {
    use clueengine::{ClueEngine, CardUtils, Card, CardType, CardSet};
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_card_type() {
        assert_eq!(CardType::Suspect, CardUtils::card_type(Card::ProfessorPlum));
        assert_eq!(CardType::Suspect, CardUtils::card_type(Card::ColonelMustard));
        assert_eq!(CardType::Suspect, CardUtils::card_type(Card::MrsPeacock));
        assert_eq!(CardType::Weapon, CardUtils::card_type(Card::Knife));
        assert_eq!(CardType::Weapon, CardUtils::card_type(Card::Wrench));
        assert_eq!(CardType::Room, CardUtils::card_type(Card::Hall));
        assert_eq!(CardType::Room, CardUtils::card_type(Card::BilliardRoom));
    }

    #[test]
    fn test_cards_of_type_suspect() {
        let expected = vec![Card::ProfessorPlum, Card::ColonelMustard, Card::MrGreen, Card::MissScarlet, Card::MsWhite, Card::MrsPeacock];
        assert_eq!(expected, CardUtils::cards_of_type(CardType::Suspect).collect::<Vec<Card>>());
    }

    #[test]
    fn test_cards_of_type_weapon() {
        let expected = vec![Card::Knife, Card::Candlestick, Card::Revolver, Card::LeadPipe, Card::Rope, Card::Wrench];
        assert_eq!(expected, CardUtils::cards_of_type(CardType::Weapon).collect::<Vec<Card>>());
    }

    #[test]
    fn test_cards_of_type_room() {
        let expected = vec![Card::Hall, Card::Conservatory, Card::DiningRoom, Card::Kitchen, Card::Study, Card::Library, Card::Ballroom, Card::Lounge, Card::BilliardRoom];
        assert_eq!(expected, CardUtils::cards_of_type(CardType::Room).collect::<Vec<Card>>());
    }

    fn make_card_set(cards: Vec<Card>) -> CardSet {
        return HashSet::from_iter(cards.iter().map(|x| *x));
    }

    fn make_usize_set(set: Vec<usize>) -> HashSet<usize> {
        return HashSet::from_iter(set.iter().map(|x| *x));
    }

    #[test]
    fn test_load_from_string_simple() {
        let clue_engine = ClueEngine::load_from_string("29A-.9-.3-.").unwrap();
        assert_eq!(3, clue_engine.player_data.len());
        assert_eq!(2, clue_engine.number_of_real_players());
        assert_eq!(Some(9), clue_engine.player_data[0].num_cards);
        assert_eq!(false, clue_engine.player_data[0].is_solution_player);
        assert_eq!(1, clue_engine.player_data[0].has_cards.len());
        assert_eq!(Some(true), clue_engine.player_data[0].has_card(Card::ProfessorPlum));
        assert_eq!(0, clue_engine.player_data[0].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[0].possible_cards.len());

        assert_eq!(Some(9), clue_engine.player_data[1].num_cards);
        assert_eq!(false, clue_engine.player_data[1].is_solution_player);
        assert_eq!(0, clue_engine.player_data[1].has_cards.len());
        assert_eq!(1, clue_engine.player_data[1].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[1].possible_cards.len());

        assert_eq!(Some(3), clue_engine.player_data[2].num_cards);
        assert_eq!(true, clue_engine.player_data[2].is_solution_player);
        assert_eq!(0, clue_engine.player_data[2].has_cards.len());
        assert_eq!(1, clue_engine.player_data[2].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[2].possible_cards.len());
    }

    #[test]
    fn test_load_from_string_has_and_not_has() {
        let clue_engine = ClueEngine::load_from_string("29A-B.9L-C.3U-.").unwrap();
        assert_eq!(Some(true), clue_engine.player_data[0].has_card(Card::ProfessorPlum));
        assert_eq!(Some(false), clue_engine.player_data[0].has_card(Card::ColonelMustard));
        assert_eq!(Some(true), clue_engine.player_data[1].has_card(Card::Wrench));
        assert_eq!(Some(false), clue_engine.player_data[1].has_card(Card::MrGreen));
        assert_eq!(Some(true), clue_engine.player_data[2].has_card(Card::BilliardRoom));
    }

    #[test]
    fn test_load_from_string_some_clauses() {
        let clue_engine = ClueEngine::load_from_string("29-.9A-B-CDE-FGH.3U-.").unwrap();
        assert_eq!(Some(true), clue_engine.player_data[1].has_card(Card::ProfessorPlum));
        assert_eq!(Some(false), clue_engine.player_data[1].has_card(Card::ColonelMustard));
        assert_eq!(2, clue_engine.player_data[1].possible_cards.len());
        assert_eq!(make_card_set(vec![Card::MrGreen, Card::MissScarlet, Card::MsWhite]), clue_engine.player_data[1].possible_cards[0]);
        assert_eq!(make_card_set(vec![Card::MrsPeacock, Card::Knife, Card::Candlestick]), clue_engine.player_data[1].possible_cards[1]);
    }

    #[test]
    fn test_load_from_string_then_write_to_string_1() {
        assert_load_from_string_then_write_to_string_match("29AH-BCD-KL-MN.9-AH.3-AH.");
    }

    #[test]
    fn test_load_from_string_then_write_to_string_2() {
        assert_load_from_string_then_write_to_string_match("29-AU.9A-BU-CDE-FH.3U-AMNOPQRST.");
    }

    fn assert_load_from_string_then_write_to_string_match(s: &str) {
        let clue_engine = ClueEngine::load_from_string(s).unwrap();
        assert_eq!(s, clue_engine.write_to_string());
    }

    #[test]
    fn test_load_from_string_does_not_start_with_number_fails() {
        if let Ok(_) = ClueEngine::load_from_string("a9-.9-.3-.") {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn test_load_from_string_player_cards_is_not_number_fails() {
        if let Ok(_) = ClueEngine::load_from_string("2a-.9-.3-.") {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn test_load_from_string_player_has_invalid_card_fails() {
        if let Ok(_) = ClueEngine::load_from_string("29Y-.9-.3-.") {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn test_load_from_string_no_dash_separating_clauses_fails() {
        if let Ok(_) = ClueEngine::load_from_string("29-.9-.3.") {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn test_load_from_string_no_ending_period_fails() {
        if let Ok(_) = ClueEngine::load_from_string("29-.9-.3-") {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn test_load_from_string_not_enough_players_fails() {
        if let Ok(_) = ClueEngine::load_from_string("39-.9-.3-") {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn test_add_card_expect_no_extras() {
        let mut clue_engine = ClueEngine::load_from_string("63FJQ-ABCDEGHIKLMNOPRSTU.3T-CDFHIJKNOPQS.3-CDFHIJKMNOPQST.3NO-CDFHIJKMPQST.3K-CDFHIJNOPQT.3CD-FJNOQT.3-CDFJNOQT.").unwrap();
        clue_engine.learn_info_on_card(6, Card::Candlestick, true, true);

        assert_eq!(true, clue_engine.is_consistent());
        assert_eq!(Some(true), clue_engine.player_data[6].has_card(Card::Candlestick));
        // This is the third green card, so Kitchen must be the solution
        assert_eq!(Some(true), clue_engine.player_data[6].has_card(Card::Kitchen));
        assert_eq!(2, clue_engine.player_data[6].has_cards.len());
    }

    #[test]
    fn test_simple_suggest() {
        let mut clue_engine = ClueEngine::new(5);

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(3), Some(Card::Knife));

        assert_eq!(Some(true), clue_engine.player_data[3].has_card(Card::Knife));
        assert_eq!(Some(false), clue_engine.player_data[4].has_card(Card::Knife));
        assert_eq!(None, clue_engine.player_data[3].has_card(Card::ProfessorPlum));
        assert_eq!(None, clue_engine.player_data[3].has_card(Card::Hall));
        assert_eq!(Some(false), clue_engine.player_data[2].has_card(Card::ProfessorPlum));
        assert_eq!(Some(false), clue_engine.player_data[2].has_card(Card::Hall));
        assert_eq!(Some(false), clue_engine.player_data[1].has_card(Card::ProfessorPlum));
        assert_eq!(Some(false), clue_engine.player_data[1].has_card(Card::Hall));
        assert_eq!(None, clue_engine.player_data[0].has_card(Card::ProfessorPlum));
        assert_eq!(None, clue_engine.player_data[0].has_card(Card::Hall));
        assert_eq!(None, clue_engine.player_data[4].has_card(Card::ProfessorPlum));
        assert_eq!(None, clue_engine.player_data[4].has_card(Card::Hall));
        assert_eq!(None, clue_engine.player_data[5].has_card(Card::ProfessorPlum));
        assert_eq!(None, clue_engine.player_data[5].has_card(Card::Hall));
    }

    #[test]
    fn test_suggest_no_refute() {
        let mut clue_engine = ClueEngine::new(3);

        clue_engine.learn_suggest(1, Card::ProfessorPlum, Card::Knife, Card::Hall, None, None);
        clue_engine.learn_info_on_card(1, Card::ProfessorPlum, false, true);

        assert_eq!(Some(true), clue_engine.player_data[clue_engine.number_of_real_players()].has_card(Card::ProfessorPlum));
        assert_eq!(Some(false), clue_engine.player_data[clue_engine.number_of_real_players()].has_card(Card::ColonelMustard));
        assert_eq!(None, clue_engine.player_data[clue_engine.number_of_real_players()].has_card(Card::Knife));
        assert_eq!(None, clue_engine.player_data[clue_engine.number_of_real_players()].has_card(Card::Hall));
        assert_eq!(Some(false), clue_engine.player_data[1].has_card(Card::ProfessorPlum));
        assert_eq!(None, clue_engine.player_data[1].has_card(Card::Knife));
        assert_eq!(Some(false), clue_engine.player_data[0].has_card(Card::Knife));
        assert_eq!(Some(false), clue_engine.player_data[2].has_card(Card::Knife));
    }

    #[test]
    fn test_possible_cards_1() {
        let mut clue_engine = ClueEngine::new(6);
        assert_eq!(0, clue_engine.player_data[3].possible_cards.len());

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(3), None);
        assert_eq!(make_usize_set(vec![0,3,4,5,6]), clue_engine.who_has_card(Card::ProfessorPlum));
        assert_eq!(1, clue_engine.player_data[3].possible_cards.len());
        assert_eq!(make_card_set(vec![Card::ProfessorPlum, Card::Knife, Card::Hall]), clue_engine.player_data[3].possible_cards[0]);

        clue_engine.learn_info_on_card(3, Card::Hall, true, true);
        assert_eq!(make_usize_set(vec![3]), clue_engine.who_has_card(Card::Hall));
        assert_eq!(Some(true), clue_engine.player_data[3].has_card(Card::Hall));
        assert_eq!(0, clue_engine.player_data[3].possible_cards.len());
    }

    #[test]
    fn test_possible_cards_2() {
        let mut clue_engine = ClueEngine::new(6);
        assert_eq!(0, clue_engine.player_data[3].possible_cards.len());

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(3), None);
        clue_engine.learn_info_on_card(3, Card::Hall, false, true);
        assert_eq!(Some(false), clue_engine.player_data[3].has_card(Card::Hall));
        assert_eq!(1, clue_engine.player_data[3].possible_cards.len());
        assert_eq!(make_card_set(vec![Card::ProfessorPlum, Card::Knife]), clue_engine.player_data[3].possible_cards[0]);
        
        clue_engine.learn_info_on_card(3, Card::ProfessorPlum, false, true);
        assert_eq!(Some(false), clue_engine.player_data[3].has_card(Card::ProfessorPlum));
        assert_eq!(make_usize_set(vec![3]), clue_engine.who_has_card(Card::Knife));
        assert_eq!(Some(true), clue_engine.player_data[3].has_card(Card::Knife));
        assert_eq!(0, clue_engine.player_data[3].possible_cards.len());
    }

    #[test]
    fn test_all_cards_accounted_for() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_info_on_card(0, Card::ColonelMustard, true, true);
        clue_engine.learn_info_on_card(1, Card::MrGreen, true, true);
        clue_engine.learn_info_on_card(2, Card::MissScarlet, true, true);
        clue_engine.learn_info_on_card(3, Card::MsWhite, true, true);
        clue_engine.learn_info_on_card(4, Card::MrsPeacock, true, true);

        assert_eq!(Some(true), clue_engine.player_data[clue_engine.number_of_real_players()].has_card(Card::ProfessorPlum));
    }

    #[test]
    fn test_single_card_accounted_for_not_solution() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_info_on_card(clue_engine.number_of_real_players(), Card::ColonelMustard, true, true);

        clue_engine.learn_info_on_card(0, Card::MrGreen, false, true);
        clue_engine.learn_info_on_card(1, Card::MrGreen, false, true);
        clue_engine.learn_info_on_card(2, Card::MrGreen, false, true);
        clue_engine.learn_info_on_card(3, Card::MrGreen, false, true);
        clue_engine.learn_info_on_card(4, Card::MrGreen, false, true);

        assert_eq!(Some(true), clue_engine.player_data[5].has_card(Card::MrGreen));
    }

    #[test]
    fn test_number_card_limit() {
        let mut clue_engine = ClueEngine::new(6);

        clue_engine.learn_info_on_card(0, Card::MrGreen, true, true);
        clue_engine.learn_info_on_card(0, Card::Knife, true, true);
        clue_engine.learn_info_on_card(0, Card::Wrench, true, true);

        assert_eq!(3, clue_engine.player_data[0].has_cards.len());
        assert_eq!(18, clue_engine.player_data[0].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[0].possible_cards.len());
    }

    #[test]
    fn test_number_card_deduction() {
        let mut clue_engine = ClueEngine::new(6);

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(2), None);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Revolver, Card::Lounge, Some(2), None);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Candlestick, Card::BilliardRoom, Some(2), None);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Rope, Card::Kitchen, Some(2), None);

        assert_eq!(make_usize_set(vec![2]), clue_engine.who_has_card(Card::ProfessorPlum));
        assert_eq!(Some(true), clue_engine.player_data[2].has_card(Card::ProfessorPlum));
    }

    #[test]
    fn test_number_card_deduction_multiple() {
        let mut clue_engine = ClueEngine::new(6);

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(2), None);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Lounge, Some(2), None);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::BilliardRoom, Some(2), None);

        assert_eq!(3, clue_engine.player_data[2].possible_cards.len());

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Kitchen, Some(2), None);
        clue_engine.learn_info_on_card(2, Card::ProfessorPlum, false, true);

        assert_eq!(make_usize_set(vec![2]), clue_engine.who_has_card(Card::Knife));
        assert_eq!(Some(true), clue_engine.player_data[2].has_card(Card::Knife));
        assert_eq!(0, clue_engine.player_data[2].possible_cards.len());
    }

    #[test]
    fn test_eliminate_extra_clauses() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(2), None);
        clue_engine.learn_info_on_card(2, Card::Hall, false, true);
        assert_eq!(1, clue_engine.player_data[2].possible_cards.len());
        assert_eq!(make_card_set(vec![Card::ProfessorPlum, Card::Knife]), clue_engine.player_data[2].possible_cards[0]);

        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Lounge, Some(2), None);

        assert_eq!(1, clue_engine.player_data[2].possible_cards.len());
        assert_eq!(make_card_set(vec![Card::ProfessorPlum, Card::Knife]), clue_engine.player_data[2].possible_cards[0]);
    }

    #[test]
    fn test_shared_clause_1() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_info_on_card(1, Card::Hall, false, true);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(1), None);
        clue_engine.learn_suggest(2, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(3), None);

        clue_engine.learn_info_on_card(3, Card::Hall, false, true);

        // No one else should have ProfessorPlum or Knife
        assert_eq!(make_usize_set(vec![1, 3]), clue_engine.who_has_card(Card::ProfessorPlum));
        assert_eq!(make_usize_set(vec![1, 3]), clue_engine.who_has_card(Card::Knife));
        assert_eq!(make_usize_set(vec![0, 2, 4, 5, 6]), clue_engine.who_has_card(Card::Hall));
    }

    #[test]
    fn test_shared_clause_2() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_info_on_card(1, Card::Hall, false, true);
        clue_engine.learn_suggest(0, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(1), None);
        clue_engine.learn_info_on_card(3, Card::Hall, false, true);

        clue_engine.learn_suggest(2, Card::ProfessorPlum, Card::Knife, Card::Hall, Some(3), None);

        // No one else should have ProfessorPlum or Knife
        assert_eq!(make_usize_set(vec![1, 3]), clue_engine.who_has_card(Card::ProfessorPlum));
        assert_eq!(make_usize_set(vec![1, 3]), clue_engine.who_has_card(Card::Knife));
        assert_eq!(make_usize_set(vec![0, 2, 4, 5, 6]), clue_engine.who_has_card(Card::Hall));
    }

    #[test]
    #[ignore] // This test is slow
    fn test_simulation_known_person_has_card() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_info_on_card(1, Card::ProfessorPlum, true, true);

        let simulation_data = clue_engine.do_simulation();
        let plum_data = simulation_data.get(&Card::ProfessorPlum).unwrap();
        assert!(plum_data[1] > 0);
        for i in 0..(clue_engine.number_of_real_players() + 1) {
            if i != 1 {
                assert_eq!(0, plum_data[i]);
            }
        }
    }

    #[test]
    #[ignore] // This test is slow
    fn test_simulation_known_person_does_not_have_card() {
        let mut clue_engine = ClueEngine::new(6);
        clue_engine.learn_info_on_card(1, Card::ProfessorPlum, false, true);

        let simulation_data = clue_engine.do_simulation();

        let plum_data = simulation_data.get(&Card::ProfessorPlum).unwrap();
        assert_eq!(0, plum_data[1]);
        for i in 0..(clue_engine.number_of_real_players() + 1) {
            if i != 1 {
                assert!(plum_data[i] > 0);
            }
        }
    }
    
    #[test]
    #[ignore] // This test is slow
    fn test_simulation_trickycase1_hasresults() {
        let clue_engine = ClueEngine::load_from_string("36CDKLQR-ABEFGHIJMNOPSTU.6T-BCDFGIKLQRS.6BF-CDKLPQRT.3-BCDFKLQRT.").unwrap();

        let simulation_data = clue_engine.do_simulation();

        // no clauses here, so all simulations should succeed
        let solution_configurations = 2*4*6;
        let expected_num_simulations = (2000 / solution_configurations) * solution_configurations;
        for card in simulation_data.keys() {
            let number_of_simulations: usize = simulation_data.get(card).unwrap().iter().sum();
            assert_eq!(expected_num_simulations, number_of_simulations);
        }
    }

    #[test]
    #[ignore] // This test is slow
    fn test_simulation_trickycase2_has_results() {
        let clue_engine = ClueEngine::load_from_string("45CPQRS-ABDEFGHIJKLMNOTU.5AGIMT-BCDEFHJKLNOPQRSU.4FK-ACDGIJLMPQRST-EO.4DL-ABCFGIJKMPQRST.3J-ACDFGHIKLMPQRST.").unwrap();

        let simulation_data = clue_engine.do_simulation();

        for card in simulation_data.keys() {
            let number_of_simulations: usize = simulation_data.get(card).unwrap().iter().sum();
            assert!(number_of_simulations > 0);
        }
    }

    #[test]
    #[ignore] // This test is slow
    fn test_simulation_clauses_point_to_card() {
        let clue_engine = ClueEngine::load_from_string("36-GM.6-GM-AHN-AIO-AJP.6-GM.3GM-HIJKLNOPQRSTU.").unwrap();

        let simulation_data = clue_engine.do_simulation();

        // In this game player with index 1 has a bunch of clauses that include Professor Plum and other cards,
        // so it's most likely he has that card
        let plum_data = simulation_data.get(&Card::ProfessorPlum).unwrap();
        let max = *plum_data.iter().max().unwrap();
        assert!(max > 0);
        assert_eq!(max, plum_data[1]);
    }
}