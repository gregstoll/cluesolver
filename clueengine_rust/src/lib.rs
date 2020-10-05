use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashSet;

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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, FromPrimitive)]
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
        // TODO - what does -1 mean anyway?
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
        s += &self.player_data.len().to_string();
        // TODO - finish
        s
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
