use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{collections::HashSet, collections::HashMap, iter::Peekable, str::Chars};
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum CardType {
    Suspect = 0,
    Weapon = 6,
    Room  = 12
}
const ALL_CARD_TYPES: [CardType; 3] = [CardType::Suspect, CardType::Weapon, CardType::Room];
 
pub struct CardUtils {
}

impl CardUtils {
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
        let mut chars = card_set.into_iter().map(|card| CardUtils::char_from_card(*card)).collect::<Vec<char>>();
        chars.sort();
        return chars.into_iter().collect();
    }


}

// https://wduquette.github.io/parsing-strings-into-slices/
/// The Tokenizer type.  
#[derive(Clone,Debug)]
pub struct Tokenizer<'a> {
    // The string being parsed.
    input: &'a str,

    // The starting index of the next character.
    index: usize,

    // The iterator used to extract characters from the input
    chars: Peekable<Chars<'a>>,
}


impl<'a> Tokenizer<'a> {
    /// Creates a new tokenizer for the given input.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            chars: input.chars().peekable(),
        }
    }

    // Returns the remainder of the input starting at the index.
    pub fn as_str(&self) -> &str {
        &self.input[self.index..]
    }

    /// Returns the next character and updates the index.
    pub fn next(&mut self) -> Option<char> {
        let ch = self.chars.next();

        if let Some(c) = ch {
            self.index += c.len_utf8();
        }

        ch
    }

    pub fn next_digit(&mut self) -> u8 {
        return self.next().unwrap().to_digit(10).unwrap() as u8;
    }

    /// Returns the next character without advancing
    pub fn peek(&mut self) -> Option<&char> {
        return self.chars.peek();
    }
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
    // None means we don't know how many cards
    num_cards: Option<u8>
}

impl PlayerData {
    //fn new(clue_engine: &'a ClueEngine<'a>, num_cards: i8, is_solution_player: bool) -> PlayerData<'a> {
    fn new(num_cards: Option<u8>, is_solution_player: bool) -> PlayerData {
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

        let num_cards_to_write = self.num_cards.unwrap_or(0);
        // Always write 0 instead of None for simplicity
        s += &num_cards_to_write.to_string();
        s += &CardUtils::card_set_to_sorted_string(&self.has_cards);
        s += "-";
        s += &CardUtils::card_set_to_sorted_string(&self.not_has_cards);
        for possible_card_group in (&self.possible_cards).into_iter() {
            s += "-";
            s += &CardUtils::card_set_to_sorted_string(&possible_card_group);
        }
        s += ".";
        return s;
    }

    fn load_from_string(self: &mut PlayerData, tokenizer: &mut Tokenizer) {
        // TODO - do we need to pass stuff back to the ClueEngine to update things?
        // seems like this shouldn't need any resolving or anything
        let num_cards = tokenizer.next_digit() as u8;
        self.num_cards = if num_cards == 0 { None } else { Some(num_cards)};
        // Load the list of cards this player has
        while *tokenizer.peek().unwrap() != '-' {
            self.info_on_card(CardUtils::card_from_char(tokenizer.next().unwrap()), true);
        }
        // advance past the '-'
        tokenizer.next();
        // Load the list of cards this player doesn't have
        {
            let mut next_char = *tokenizer.peek().unwrap();
            while next_char != '-' && next_char != '.' {
                self.info_on_card(CardUtils::card_from_char(tokenizer.next().unwrap()), true);
                next_char = *tokenizer.peek().unwrap();
            }
        }
        // Load the list of clauses as long as it's not done
        // TODO - assert this is '-' if it's not '.'?
        while tokenizer.next().unwrap() != '.' {
            let mut clause = HashSet::new();
            let mut next_char = *tokenizer.peek().unwrap();
            while next_char != '-' && next_char != '.' {
                clause.insert(CardUtils::card_from_char(tokenizer.next().unwrap()));
                next_char = *tokenizer.peek().unwrap();
            }
            if !clause.is_empty() {
                self.has_one_of_cards(clause);
            }
        }
    }

    //TODO - need a better naming scheme
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
                // TODO - add utility method for this
                other_changed_cards.iter().for_each(|c| {changed_cards.insert(*c);});
            } else {
                self.possible_cards.push(new_clause);
            }
            let updated_cards = self.examine_clauses(None);
            updated_cards.iter().for_each(|c| {changed_cards.insert(*c);});
        }
        return changed_cards;
    }

    fn has_card(self: &PlayerData, card: Card) -> Option<bool> {
        if self.has_cards.contains(&card) {
            return Some(true);
        }
        if self.not_has_cards.contains(&card) {
            return Some(false);
        }
        return None;
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
            let player_data = PlayerData::new(Some(ClueEngine::number_of_player_cards(i, number_of_players)), i == number_of_players);
            player_datas.push(player_data);
        }
        let clue_engine = ClueEngine { player_data: player_datas };
        //TODO?
        /*for i in 0..number_of_players {
            clue_engine.player_data[i].clue
        }*/
        return clue_engine;
    }

    fn number_of_real_players(self: &Self) -> u8 {
        // don't include the solution player
        return (self.player_data.len() - 1) as u8;
    }

    fn solution_player(self: &Self) -> &PlayerData {
        &self.player_data[self.player_data.len()]
    }

    fn solution_player_mut(self: &mut Self) -> &mut PlayerData {
        let len = self.player_data.len();
        &mut self.player_data[len]
    }

    fn number_of_player_cards(player_index: u8, num_players: u8) -> u8 {
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
        return num_cards as u8;
    }

    fn write_to_string(self: &ClueEngine) -> String {
        let mut s = String::from("");
        s += &(self.number_of_real_players()).to_string();
        for player in self.player_data.iter() {
            s += &player.write_to_string();
        }
        return s;
    }

    fn load_from_string(s: &str) -> ClueEngine {
        let mut tokenizer = Tokenizer::new(s);
        let number_of_players = tokenizer.next_digit();
        let mut clue_engine = ClueEngine::new(number_of_players);
        for i in 0..(number_of_players+1) {
            let player = &mut clue_engine.player_data[i as usize];
            player.load_from_string(&mut tokenizer);
        }
        return clue_engine;
    }
    
    // TODO - document this
    fn check_solution(self: &mut Self, card: Option<Card>) -> CardSet {
        // TODO - this method is really long
        let mut changed_cards: CardSet = HashSet::new();
        if let Some(real_card) = card {
            let mut someone_has_card = false;
            let mut skip_deduction = false;
            let mut number_who_dont_have_card = 0;
            let mut player_who_might_have_card = None;
            // - Check also for all cards except one in a category are
            // accounted for.
            for i in 0..self.player_data.len() {
                let player = &mut self.player_data[i];
                let has_card = player.has_card(real_card);
                match has_card {
                    Some(true) => {
                        // Someone has the card, so the solution is not this.
                        someone_has_card = true;
                        break;
                    },
                    Some(false) => {
                        number_who_dont_have_card += 1;
                    },
                    None => {
                        if player_who_might_have_card == None {
                            player_who_might_have_card = Some(i);
                        } else {
                            // The solution is not this, but someone might still
                            // have it.
                            skip_deduction = true;
                        }
                    }
                }
            }
            if !skip_deduction && !someone_has_card && number_who_dont_have_card == self.number_of_real_players() {
                // Every player except one doesn't have this card, so we know the player has it.
                let other_changed_cards = self.player_data[player_who_might_have_card.unwrap()].info_on_card(real_card, true);
                other_changed_cards.iter().for_each(|c| {changed_cards.insert(*c);});
            }
            else if someone_has_card {
                // Someone has this card, so no one else does. (including solution)
                for player in self.player_data.iter_mut() {
                    if player.has_card(real_card) == None {
                        let other_changed_cards = player.info_on_card(real_card, false);
                        other_changed_cards.iter().for_each(|c| {changed_cards.insert(*c);});
                    }
                }
            }
        }

        for card_type in ALL_CARD_TYPES.iter() {
            let all_cards = CardUtils::cards_of_type(*card_type).collect::<Vec<Card>>();
            let mut solution_card: Option<Card> = None;
            let mut is_solution = true;
            for test_card in all_cards.iter() {
                // See if anyone has this card
                let mut card_owned = false;
                for player in self.player_data.iter() {
                    if player.has_card(*test_card) == Some(true) {
                        // someone has it, mark it as such
                        card_owned = true;
                    }
                }
                if !card_owned {
                    // If there's another possibility, we don't know which is
                    // right.
                    if solution_card != None {
                        solution_card = None;
                        is_solution = false;
                    } else {
                        solution_card = Some(*test_card);
                    }
                }
            }
            if is_solution && solution_card != None {
                // There's only one possibility, so this must be it!
                let solution = solution_card.unwrap();
                if self.solution_player().has_card(solution) == None {
                    // also check to make sure we don't have another one in this category
                    // TODO - should we assert if this happens?
                    if all_cards.iter().all(|c| !self.solution_player().has_cards.contains(c)) {
                        self.solution_player_mut().has_cards.insert(solution);
                        changed_cards.insert(solution);
                    }
                }
            }
        }
        // Finally, see if any people share clauses in common.
        let mut clause_hash: HashMap<String, Vec<u8>> = HashMap::new();
        for idx in 0..self.number_of_real_players() {
            let player = &self.player_data[idx as usize];
            for clause in player.possible_cards.iter() {
                let clause_str = CardUtils::card_set_to_sorted_string(clause);
                if clause_hash.contains_key(&clause_str) {
                    clause_hash.get_mut(&clause_str).unwrap().push(idx);
                }
                else {
                    clause_hash.insert(clause_str, vec![idx]);
                }
            }
        }
        for (clause, players) in clause_hash.iter() {
            // If n people all have an n-length clause, no one else can have
            // a card in that clause.
            if (clause.len() <= players.len()) {
                let affected_people: HashSet<u8> = HashSet::from_iter(players.iter().map(|x| *x));
                for card in clause.chars().map(|ch| CardUtils::card_from_char(ch)) {
                    changed_cards.insert(card);
                }
                for idx in 0..(self.number_of_real_players() + 1) {
                    if !affected_people.contains(&idx) {
                        for card in clause.chars().map(|ch| CardUtils::card_from_char(ch)) {
                            if self.player_data[idx as usize].has_card(card) != Some(false) {
                                // TODOTODO
                                let other_changed_cards = self.player_data[idx as usize].info_on_card(card, false);
                                // TODO - add utility method for this
                                other_changed_cards.iter().for_each(|c| {changed_cards.insert(*c);});
                            }
                        }
                    }
                }
            }
        }
        return changed_cards;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_from_card() {
        assert_eq!('A', CardUtils::char_from_card(Card::ProfessorPlum));
        assert_eq!('B', CardUtils::char_from_card(Card::ColonelMustard));
        assert_eq!('F', CardUtils::char_from_card(Card::MrsPeacock));
        assert_eq!('G', CardUtils::char_from_card(Card::Knife));
        assert_eq!('L', CardUtils::char_from_card(Card::Wrench));
        assert_eq!('M', CardUtils::char_from_card(Card::Hall));
        assert_eq!('U', CardUtils::char_from_card(Card::BilliardRoom));
        for i in ('A' as u8)..('V' as u8) {
            let ch = i as char;
            assert_eq!(ch, CardUtils::char_from_card(CardUtils::card_from_char(ch)));
        }
    }

    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_below_a__panics() {
        let _ch = CardUtils::card_from_char('0');
    }
    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_above_u__panics() {
        let _ch = CardUtils::card_from_char('V');
    }

    #[test]
    fn test_card_from_char() {
        assert_eq!(Card::ProfessorPlum, CardUtils::card_from_char('A'));
        assert_eq!(Card::ColonelMustard, CardUtils::card_from_char('B'));
        assert_eq!(Card::MrsPeacock, CardUtils::card_from_char('F'));
        assert_eq!(Card::Knife, CardUtils::card_from_char('G'));
        assert_eq!(Card::Wrench, CardUtils::card_from_char('L'));
        assert_eq!(Card::Hall, CardUtils::card_from_char('M'));
        assert_eq!(Card::BilliardRoom, CardUtils::card_from_char('U'));
    }
 
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
    fn test_card_set_to_sorted_string() {
        assert_eq!("ABC", CardUtils::card_set_to_sorted_string(&vec![Card::ColonelMustard, Card::ProfessorPlum, Card::MrGreen].into_iter().collect()));
        assert_eq!("", CardUtils::card_set_to_sorted_string(&HashSet::new()));
        assert_eq!("CLU", CardUtils::card_set_to_sorted_string(&vec![Card::BilliardRoom, Card::Wrench, Card::MrGreen].into_iter().collect()));
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
        //TODO - this should be 1 once we're doing inference correctly
        assert_eq!(0, clue_engine.player_data[1].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[1].possible_cards.len());

        assert_eq!(Some(3), clue_engine.player_data[2].num_cards);
        assert_eq!(true, clue_engine.player_data[2].is_solution_player);
        assert_eq!(0, clue_engine.player_data[2].has_cards.len());
        //TODO - this should be 1 once we're doing inference correctly
        assert_eq!(0, clue_engine.player_data[2].not_has_cards.len());
        assert_eq!(0, clue_engine.player_data[2].possible_cards.len());
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
