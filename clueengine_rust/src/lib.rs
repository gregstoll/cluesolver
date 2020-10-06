use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashSet;
use std::iter::FromIterator;

type CardSet = HashSet<Card>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, FromPrimitive, Hash, Copy, Clone)]
enum Card {
    // suspects
    ProfessorPlum,
    ColonelMustard,
    MrGreen,
    MissScarlet,
    MsWhite,
    MrsPeacock,
    // weapons
    Knife,
    Candlestick,
    Revolver,
    LeadPipe,
    Rope,
    Wrench,
    // rooms
    Hall,
    Conservatory,
    DiningRoom,
    Kitchen,
    Study,
    Library,
    Ballroom,
    Lounge,
    BilliardRoom
}
const CARD_LAST : i32 = (Card::BilliardRoom as i32) + 1;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum CardType {
    Suspect = 0,
    Weapon = 6,
    Room  = 12
}
 
pub struct PlayerData {
    // A set of cards that the player is known to have
    has_cards: CardSet,
    // A set of cards that the player is known not to have
    not_has_cards: CardSet,
    // A list of clauses.  Each clause is a set of cards, one of which
    // the player is known to have.
    possible_cards: Vec<CardSet>,
    //clue_engine: &'a ClueEngine<'a>, // TODO - do we really need this?
    is_solution_player: bool,
    num_cards: i8
}

impl PlayerData {
    //fn new(clue_engine: &'a ClueEngine<'a>, num_cards: i8, is_solution_player: bool) -> PlayerData<'a> {
    fn new(num_cards: i8, is_solution_player: bool) -> PlayerData {
        return PlayerData {
            has_cards: HashSet::new(),
            not_has_cards: HashSet::new(),
            possible_cards: vec!(),
            //clue_engine,
            is_solution_player,
            num_cards
        };
    }

    fn write_to_string(self: &PlayerData) -> String {
        let mut s = String::from("");

        let mut num_cards_to_write = self.num_cards;
        // Always write one digit for simplicity
        // TODO - what does -1 mean anyway?  (maybe unknown?)
        if num_cards_to_write == -1 {
            num_cards_to_write = 0;
        }
        s += &num_cards_to_write.to_string();
        s += &ClueEngine::card_set_to_sorted_string(&self.has_cards);
        s += "-";
        s += &ClueEngine::card_set_to_sorted_string(&self.not_has_cards);
        for possible_card_group in (&self.possible_cards).into_iter() {
            s += "-";
            s += &ClueEngine::card_set_to_sorted_string(&possible_card_group);
        }
        s += ".";
        return s;
    }

    fn load_from_string(self: &mut PlayerData, mut s: &str) {
        // TODO - do we need to pass stuff back to the ClueEngine to update things?
        // seems like this shouldn't need any resolving or anything
        self.num_cards = s[0..1].parse::<i8>().unwrap();
        if self.num_cards == 0 {
            self.num_cards = -1;
        }
        s = &s[1..];
        // Load the list of cards this player has
        // TODO - this string manipulation could maybe be refactored/improved?
        while s.chars().next().unwrap() != '-' {
            self.info_on_card(ClueEngine::card_from_char(s.chars().next().unwrap()), true);
            s = &s[1..];
        }
        s = &s[1..];
        // Load the list of cards this player doesn't have
        {
            let mut next_char = s.chars().next().unwrap();
            while next_char != '-' && next_char != '.' {
                self.info_on_card(ClueEngine::card_from_char(s.chars().next().unwrap()), true);
                s = &s[1..];
                next_char = s.chars().next().unwrap();
            }
        }
        // Load the list of clauses as long as it's not done
        while s.chars().next().unwrap() != '.' {
            s = &s[1..];
            let mut clause = HashSet::new();
            let mut next_char = s.chars().next().unwrap();
            while next_char != '-' && next_char != '.' {
                clause.insert(ClueEngine::card_from_char(next_char));
                s = &s[1..];
                next_char = s.chars().next().unwrap();
            }
            if !clause.is_empty() {
                self.has_one_of_cards(clause);
            }
        }
        s = &s[1..];
    }

    fn has_one_of_cards(self: &mut PlayerData, cards: CardSet) -> CardSet {
        let mut clause_helpful = true;
        let mut changed_cards = HashSet::new();
        let mut new_clause = HashSet::new();
        //TODO finish
        for card in cards.iter() {
            if self.has_cards.contains(card) {
                // We already know player has one of these cards, so this
                // clause is worthless.
                clause_helpful = false;
            }
            else if self.not_has_cards.contains(card) {
                // We know player doesn't have this card, so don't add this card
                // to the new clause.
            }
            else {
                // Don't know; add it to the new clause
                new_clause.insert(*card);
            }
        }
        if clause_helpful && !new_clause.is_empty() {
            if new_clause.len() == 1 {
                // We have learned player has this card!
                let new_card = *new_clause.iter().next().unwrap();
                let other_changed_cards = self.info_on_card(new_card, true);
                other_changed_cards.iter().for_each(|c| {changed_cards.insert(*c);});
            } else {
                self.possible_cards.push(new_clause);
            }
            let updated_cards = self.examine_clauses(None);
            updated_cards.iter().for_each(|c| {changed_cards.insert(*c);});
        }
        return changed_cards;
    }

    // TODO - updateClueEngine stuff?
    fn info_on_card(self: &mut PlayerData, card: Card, has_card: bool) -> CardSet {
        let mut changed_cards = HashSet::new();
        if has_card {
            self.has_cards.insert(card);
        }
        else {
            self.not_has_cards.insert(card);
        }
        changed_cards.insert(card);
        //TODO more

        return changed_cards;
    }

    fn examine_clauses(self: &mut PlayerData, card: Option<Card>) -> CardSet{
        //TODO
        return HashSet::new();
    }

    fn eliminate_extraneous_clauses(self: &mut PlayerData) {
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut self.possible_cards);
    }
    fn eliminate_extraneous_clauses_possible_cards(possible_cards: &mut Vec<CardSet>) {
        let mut need_to_call_again = false;
        // This is O(n^2), but hopefully there aren't too many of these
        'outer: for i in 0..possible_cards.len() {
            for j in (i+1)..possible_cards.len() {
                let clause_1 = &possible_cards[i];
                let clause_2 = &possible_cards[j];
                if clause_1.is_subset(clause_2) {
                    // clause_2 is extraneous
                    possible_cards.remove(j);
                    need_to_call_again = true;
                    break 'outer;
                }
                else if clause_1.is_superset(clause_2) {
                    // clause_1 is extraneous
                    possible_cards.remove(i);
                    need_to_call_again = true;
                    break 'outer;
                }
            }
        }
        if need_to_call_again {
            // The easiest way to check without messing up the loop is
            // to start over, although it's kinda slow.  But I don't
            // expect there to be tons of extraneous clauses.
            PlayerData::eliminate_extraneous_clauses_possible_cards(possible_cards);
        }
    }
}

pub struct ClueEngine {
    player_data: Vec<PlayerData>,
}

impl ClueEngine {
    fn new(number_of_players: u8) -> ClueEngine {
        let mut player_datas: Vec<PlayerData> = vec!();
        for i in 0..(number_of_players + 1) {
            let player_data = PlayerData::new(ClueEngine::number_of_player_cards(i, number_of_players), i == number_of_players);
            player_datas.push(player_data);
        }
        let clue_engine = ClueEngine { player_data: player_datas };
        //TODO?
        /*for i in 0..number_of_players {
            clue_engine.player_data[i].clue
        }*/
        return clue_engine;
    }

    fn card_from_char(ch: char) -> Card {
        let index = ch as u8 - 'A' as u8;
        return FromPrimitive::from_u8(index).unwrap()
    }

    fn char_from_card(card: Card) -> char {
        let index = card as u8 + 'A' as u8;
        return index as char;
    }

    fn card_type(card: Card) -> CardType {
        let index = card as u8;
        if index < CardType::Weapon as u8 {
            return CardType::Suspect;
        }
        if index < CardType::Room as u8 {
            return CardType::Weapon;
        }
        return CardType::Room;
    }

    fn cards_of_type(card_type: CardType) -> impl Iterator<Item=Card> {
        let int_range = match card_type {
            CardType::Suspect => (CardType::Suspect as u8)..(CardType::Weapon as u8),
            CardType::Weapon => (CardType::Weapon as u8)..(CardType::Room as u8),
            CardType::Room => (CardType::Room as u8)..(CARD_LAST as u8),
        };
        return int_range.map(|x| FromPrimitive::from_u8(x).unwrap());
    }
    
    fn card_set_to_sorted_string(card_set: &CardSet) -> String {
        let mut chars = card_set.into_iter().map(|card| ClueEngine::char_from_card(*card)).collect::<Vec<char>>();
        chars.sort();
        return chars.into_iter().collect();
    }

    fn number_of_player_cards(player_index: u8, num_players: u8) -> i8 {
        if player_index == num_players {
            // The case file always has exactly 3 cards
            return 3
        }
        // There are 18 cards among the players.
        let mut num_cards = 18 / num_players; // Integer division
        let leftovers = 18 % num_players;
        // Assume the earlier players get the extra cards
        if player_index < leftovers {
            num_cards += 1;
        }
        return num_cards as i8;
    }

    fn write_to_string(self: &ClueEngine) -> String {
        let mut s = String::from("");
        s += &(self.player_data.len() - 1).to_string();
        for player in self.player_data.iter() {
            s += &player.write_to_string();
        }
        return s;
    }

    fn load_from_string(mut s: &str) -> ClueEngine {
        let number_of_players = s[0..1].parse::<u8>().unwrap();
        let mut clue_engine = ClueEngine::new(number_of_players);
        s = &s[1..];
        for i in 0..(number_of_players+1) {
            let mut player = &mut clue_engine.player_data[i as usize];
            player.load_from_string(s);
        }
        return clue_engine;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_from_card() {
        assert_eq!('A', ClueEngine::char_from_card(Card::ProfessorPlum));
        assert_eq!('B', ClueEngine::char_from_card(Card::ColonelMustard));
        assert_eq!('F', ClueEngine::char_from_card(Card::MrsPeacock));
        assert_eq!('G', ClueEngine::char_from_card(Card::Knife));
        assert_eq!('L', ClueEngine::char_from_card(Card::Wrench));
        assert_eq!('M', ClueEngine::char_from_card(Card::Hall));
        assert_eq!('U', ClueEngine::char_from_card(Card::BilliardRoom));
        for i in ('A' as u8)..('V' as u8) {
            let ch = i as char;
            assert_eq!(ch, ClueEngine::char_from_card(ClueEngine::card_from_char(ch)));
        }
    }

    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_below_a__panics() {
        let _ch = ClueEngine::card_from_char('0');
    }
    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_above_u__panics() {
        let _ch = ClueEngine::card_from_char('V');
    }

    #[test]
    fn test_card_from_char() {
        assert_eq!(Card::ProfessorPlum, ClueEngine::card_from_char('A'));
        assert_eq!(Card::ColonelMustard, ClueEngine::card_from_char('B'));
        assert_eq!(Card::MrsPeacock, ClueEngine::card_from_char('F'));
        assert_eq!(Card::Knife, ClueEngine::card_from_char('G'));
        assert_eq!(Card::Wrench, ClueEngine::card_from_char('L'));
        assert_eq!(Card::Hall, ClueEngine::card_from_char('M'));
        assert_eq!(Card::BilliardRoom, ClueEngine::card_from_char('U'));
    }
 
    #[test]
    fn test_card_type() {
        assert_eq!(CardType::Suspect, ClueEngine::card_type(Card::ProfessorPlum));
        assert_eq!(CardType::Suspect, ClueEngine::card_type(Card::ColonelMustard));
        assert_eq!(CardType::Suspect, ClueEngine::card_type(Card::MrsPeacock));
        assert_eq!(CardType::Weapon, ClueEngine::card_type(Card::Knife));
        assert_eq!(CardType::Weapon, ClueEngine::card_type(Card::Wrench));
        assert_eq!(CardType::Room, ClueEngine::card_type(Card::Hall));
        assert_eq!(CardType::Room, ClueEngine::card_type(Card::BilliardRoom));
    }

    #[test]
    fn test_card_set_to_sorted_string() {
        assert_eq!("ABC", ClueEngine::card_set_to_sorted_string(&vec![Card::ColonelMustard, Card::ProfessorPlum, Card::MrGreen].into_iter().collect()));
        assert_eq!("", ClueEngine::card_set_to_sorted_string(&HashSet::new()));
        assert_eq!("CLU", ClueEngine::card_set_to_sorted_string(&vec![Card::BilliardRoom, Card::Wrench, Card::MrGreen].into_iter().collect()));
    }

    #[test]
    fn test_cards_of_type_suspect() {
        let expected = vec![Card::ProfessorPlum, Card::ColonelMustard, Card::MrGreen, Card::MissScarlet, Card::MsWhite, Card::MrsPeacock];
        assert_eq!(expected, ClueEngine::cards_of_type(CardType::Suspect).collect::<Vec<Card>>());
    }

    #[test]
    fn test_cards_of_type_weapon() {
        let expected = vec![Card::Knife, Card::Candlestick, Card::Revolver, Card::LeadPipe, Card::Rope, Card::Wrench];
        assert_eq!(expected, ClueEngine::cards_of_type(CardType::Weapon).collect::<Vec<Card>>());
    }

    #[test]
    fn test_cards_of_type_room() {
        let expected = vec![Card::Hall, Card::Conservatory, Card::DiningRoom, Card::Kitchen, Card::Study, Card::Library, Card::Ballroom, Card::Lounge, Card::BilliardRoom];
        assert_eq!(expected, ClueEngine::cards_of_type(CardType::Room).collect::<Vec<Card>>());
    }
    #[test]
    fn test_eliminate_extraneous_clauses_empty() {
        let mut clauses: Vec<CardSet> = vec![];
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert!(clauses.is_empty());
    }

    fn make_card_set(cards: Vec<Card>) -> CardSet {
        return HashSet::from_iter(cards.iter().map(|x| *x));
    }

    #[test]
    fn test_eliminate_extraneous_clauses_single() {
        let mut clauses: Vec<CardSet> = vec![make_card_set(vec![Card::ProfessorPlum, Card::MsWhite])];
        let expected = clauses.clone();
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert_eq!(expected, clauses);
    }

    #[test]
    fn test_eliminate_extraneous_clauses_three_not_extraneous() {
        let mut clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Library, Card::Wrench]),
            make_card_set(vec![Card::Conservatory, Card::MsWhite])];
        let expected = clauses.clone();
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert_eq!(expected, clauses);
    }

    #[test]
    fn test_eliminate_extraneous_clauses_superset_first() {
        let mut clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Library, Card::Wrench, Card::Conservatory]),
            make_card_set(vec![Card::Conservatory, Card::Wrench]),
            make_card_set(vec![Card::Library, Card::Hall])];
        let mut expected = clauses.clone();
        expected.remove(1);
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert_eq!(expected, clauses);
    }

    #[test]
    fn test_eliminate_extraneous_clauses_subset_first() {
        let mut clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Conservatory, Card::Wrench]),
            make_card_set(vec![Card::Library, Card::Wrench, Card::Conservatory]),
            make_card_set(vec![Card::Library, Card::Hall])];
        let mut expected = clauses.clone();
        expected.remove(2);
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert_eq!(expected, clauses);
    }

    #[test]
    fn test_eliminate_extraneous_clauses_multiple_redundant_to_same_one() {
        let mut clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Conservatory, Card::Wrench]),
            make_card_set(vec![Card::Library, Card::Wrench, Card::Conservatory]),
            make_card_set(vec![Card::Library, Card::Hall]),
            make_card_set(vec![Card::Wrench, Card::Library]),
            ];
        let mut expected = clauses.clone();
        expected.remove(2);
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert_eq!(expected, clauses);
    }

    #[test]
    fn test_eliminate_extraneous_clauses_multiple_redundant_to_different_ones() {
        let mut clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Conservatory, Card::Wrench]),
            make_card_set(vec![Card::Library, Card::Wrench, Card::Conservatory]),
            make_card_set(vec![Card::Library, Card::Hall]),
            make_card_set(vec![Card::Wrench, Card::Library]),
            make_card_set(vec![Card::MrGreen, Card::Wrench, Card::BilliardRoom]),
            make_card_set(vec![Card::BilliardRoom, Card::Wrench]),
            ];
        let mut expected = clauses.clone();
        expected.remove(5);
        expected.remove(2);
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert_eq!(expected, clauses);
    }

    #[test]
    fn test_load_from_string_simple() {
        let clue_engine = ClueEngine::load_from_string("29A-.9-.3-.");
        assert_eq!(3, clue_engine.player_data.len());
        //TODO - write PlayerData::has_card() that returns an Option<bool> here
        assert_eq!(1, clue_engine.player_data[0].has_cards.len());
        assert!(clue_engine.player_data[0].has_cards.contains(&Card::ProfessorPlum));
        assert_eq!(0, clue_engine.player_data[0].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[0].possible_cards.len());
        //TODO - finish
    }


    #[test]
    fn test_number_of_cards() {
        // solution files
        assert_eq!(3, ClueEngine::number_of_player_cards(3, 3));
        assert_eq!(3, ClueEngine::number_of_player_cards(4, 4));
        assert_eq!(3, ClueEngine::number_of_player_cards(5, 5));
        assert_eq!(3, ClueEngine::number_of_player_cards(6, 6));

        assert_eq!(6, ClueEngine::number_of_player_cards(0, 3));
        assert_eq!(6, ClueEngine::number_of_player_cards(1, 3));
        assert_eq!(6, ClueEngine::number_of_player_cards(2, 3));

        assert_eq!(5, ClueEngine::number_of_player_cards(0, 4));
        assert_eq!(5, ClueEngine::number_of_player_cards(1, 4));
        assert_eq!(4, ClueEngine::number_of_player_cards(2, 4));
        assert_eq!(4, ClueEngine::number_of_player_cards(3, 4));

        assert_eq!(4, ClueEngine::number_of_player_cards(0, 5));
        assert_eq!(4, ClueEngine::number_of_player_cards(1, 5));
        assert_eq!(4, ClueEngine::number_of_player_cards(2, 5));
        assert_eq!(3, ClueEngine::number_of_player_cards(3, 5));
        assert_eq!(3, ClueEngine::number_of_player_cards(4, 5));

        assert_eq!(3, ClueEngine::number_of_player_cards(0, 6));
        assert_eq!(3, ClueEngine::number_of_player_cards(1, 6));
        assert_eq!(3, ClueEngine::number_of_player_cards(2, 6));
        assert_eq!(3, ClueEngine::number_of_player_cards(3, 6));
        assert_eq!(3, ClueEngine::number_of_player_cards(4, 6));
        assert_eq!(3, ClueEngine::number_of_player_cards(5, 6));
    }
}
