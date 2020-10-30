use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{collections::HashSet, collections::HashMap, iter::Peekable, str::Chars};
use std::cmp::min;
use std::iter::FromIterator;
use rayon::prelude::*;

pub type CardSet = HashSet<Card>;
pub type SimulationData = HashMap<Card, Vec<usize>>;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, FromPrimitive, Hash, Copy, Clone)]
pub enum Card {
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
pub enum CardType {
    Suspect = 0,
    Weapon = 6,
    Room  = 12
}
//TODO - document these
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum UpdateEngineMode {
    None,
    Minimal,
    All
}

impl From<bool> for UpdateEngineMode {
    fn from(b: bool) -> Self {
        if b { UpdateEngineMode::All } else { UpdateEngineMode::Minimal }
    }
}

pub struct CardUtils {
}

impl CardUtils {
    pub fn card_from_char(ch: char) -> Result<Card, String> {
        let index = ch as i32 - 'A' as i32;
        if index < 0 || index >= CARD_LAST {
            return Err(format!("Invalid card character '{}'", ch));
        }
        return FromPrimitive::from_i32(index).ok_or(format!("Invalid card character '{}'", ch));
    }

    pub fn char_from_card(card: Card) -> char {
        let index = card as u8 + 'A' as u8;
        return index as char;
    }

    pub fn card_type(card: Card) -> CardType {
        let index = card as u8;
        if index < CardType::Weapon as u8 {
            return CardType::Suspect;
        }
        if index < CardType::Room as u8 {
            return CardType::Weapon;
        }
        return CardType::Room;
    }

    pub fn all_cards() -> impl Iterator<Item=Card> {
        return (0..CARD_LAST).map(|x| FromPrimitive::from_i32(x).unwrap());
    }

    pub fn all_card_types() -> impl Iterator<Item=&'static CardType> {
        const ALL_CARD_TYPES: [CardType; 3] = [CardType::Suspect, CardType::Weapon, CardType::Room];
        return ALL_CARD_TYPES.iter();
    }

    pub fn cards_of_type(card_type: CardType) -> impl Iterator<Item=Card> {
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
struct Tokenizer<'a> {
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

    /// Returns the next character and updates the index.
    pub fn next(&mut self) -> Option<char> {
        let ch = self.chars.next();

        if let Some(c) = ch {
            self.index += c.len_utf8();
        }

        ch
    }

    pub fn next_digit(&mut self) -> Result<u8, ()> {
        return Ok(self.next().ok_or(())?.to_digit(10).ok_or(())? as u8);
    }

    // Returns the remainder of the input starting at the index.
    pub fn as_str(&self) -> &str {
        &self.input[self.index..]
    }

    /// Returns the next character without advancing
    pub fn peek(&mut self) -> Option<&char> {
        return self.chars.peek();
    }
}


#[derive(Clone,Debug)]
pub struct PlayerData {
    // A set of cards that the player is known to have
    pub has_cards: CardSet,
    // A set of cards that the player is known not to have
    pub not_has_cards: CardSet,
    // A list of clauses.  Each clause is a set of cards, one of which
    // the player is known to have.
    pub possible_cards: Vec<CardSet>,
    pub is_solution_player: bool,
    // None means we don't know how many cards
    pub num_cards: Option<u8>
}

impl PlayerData {
    pub fn new(num_cards: Option<u8>, is_solution_player: bool) -> PlayerData {
        return PlayerData {
            has_cards: HashSet::new(),
            not_has_cards: HashSet::new(),
            possible_cards: vec!(),
            is_solution_player,
            num_cards
        };
    }

    pub fn write_to_string(self: &PlayerData) -> String {
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


    pub fn has_card(self: &PlayerData, card: Card) -> Option<bool> {
        if self.has_cards.contains(&card) {
            return Some(true);
        }
        if self.not_has_cards.contains(&card) {
            return Some(false);
        }
        return None;
    }


    pub fn eliminate_extraneous_clauses(self: &mut PlayerData) {
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut self.possible_cards);
    }
    pub fn eliminate_extraneous_clauses_possible_cards(possible_cards: &mut Vec<CardSet>) {
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

#[derive(Debug, Clone)]
pub struct ClueEngine {
    pub player_data: Vec<PlayerData>,
}

impl ClueEngine {
    pub const NUM_SIMULATIONS: i32 = 20000;

    pub fn new(number_of_players: u8, number_of_cards_per_player: Option<&Vec<u8>>) -> Result<ClueEngine, String> {
        let real_cards_per_player: &Vec<u8>;
        let allocated_cards_per_player: Vec<u8>;
        if let Some(vec) = number_of_cards_per_player {
            real_cards_per_player = vec;
        }
        else {
            allocated_cards_per_player = (0..number_of_players).map(|i| ClueEngine::number_of_player_cards(i, number_of_players)).collect();
            real_cards_per_player = &allocated_cards_per_player;
        }
        if real_cards_per_player.len() != number_of_players as usize {
            return Err(format!("Wrong number of cards in number_of_cards_per_player vector! (expected {}, got {})", number_of_players, real_cards_per_player.len()));
        }
        // There are CARD_LAST - 3 (actually 18) cards among the players.
        if real_cards_per_player.iter().sum::<u8>() != CARD_LAST as u8 - 3 {
            return Err(format!("Wrong total number of cards in number_of_cards_per_player! (expected {}, got {})", CARD_LAST - 3, real_cards_per_player.iter().sum::<u8>()));

        }
        let mut player_datas: Vec<PlayerData> = vec!();
        for i in 0..(number_of_players + 1) {
            let number_of_cards;
            if i == number_of_players {
                number_of_cards = 3 as u8;
            }
            else {
                number_of_cards = real_cards_per_player[i as usize];
            }
            let player_data = PlayerData::new(Some(number_of_cards), i == number_of_players);
            player_datas.push(player_data);
        }
        Ok(ClueEngine { player_data: player_datas })
    }

    pub fn number_of_real_players(self: &Self) -> usize {
        // don't include the solution player
        return self.player_data.len() - 1;
    }

    pub fn solution_player(self: &Self) -> &PlayerData {
        &self.player_data[self.number_of_real_players()]
    }

    pub fn solution_player_mut(self: &mut Self) -> &mut PlayerData {
        let index = self.number_of_real_players();
        &mut self.player_data[index]
    }

    pub fn number_of_player_cards(player_index: u8, num_players: u8) -> u8 {
        if player_index == num_players {
            // The case file always has exactly 3 cards
            return 3
        }
        // There are 18 cards among the players.
        // There are CARD_LAST - 3 (actually 18) cards among the players.
        let mut num_cards = (CARD_LAST - 3) as u8 / num_players; // Integer division
        let leftovers = (CARD_LAST - 3) as u8 % num_players;
        // Assume the earlier players get the extra cards
        if player_index < leftovers {
            num_cards += 1;
        }
        return num_cards as u8;
    }

    pub fn write_to_string(self: &ClueEngine) -> String {
        let mut s = String::from("");
        s += &(self.number_of_real_players()).to_string();
        for player in self.player_data.iter() {
            s += &player.write_to_string();
        }
        return s;
    }

    pub fn load_from_string(s: &str) -> Result<ClueEngine, String> {
        let mut tokenizer = Tokenizer::new(s);
        let number_of_players = tokenizer.next_digit().map_err(|_| String::from("Error - couldn't parse number of players!"))?;
        // This can't fail because we're not specifying the number of cards, so they'll get set correctly.
        let mut clue_engine = ClueEngine::new(number_of_players, None).unwrap();
        for i in 0..(number_of_players+1) {
            clue_engine.load_player_from_string(i as usize, &mut tokenizer)?;
        }
        // Ensure we've consumed all of the input
        if tokenizer.peek() == None {
            return Ok(clue_engine);
        }
        else {
            return Err(format!("Didn't use all of string; the part that was left is \"{}\"", tokenizer.as_str()));
        }
    }

    // format is (concatenated)
    // <number of cards (or 0 if this is unknown)>
    // one letter per card in has_cards
    // '-'
    // one letter per card in not_has_cards
    // '-'
    // one letter per card in possible_clauses
    //  (each possible_clause is separated by '-')
    // '.'
    fn load_player_from_string(self: &mut ClueEngine, player_index: usize, tokenizer: &mut Tokenizer) -> Result<(), String> {
        const STRING_ENDED_ERROR: &str = "string ended unexpectedly!";
        {
            let num_cards = tokenizer.next_digit().map_err(|_| String::from(STRING_ENDED_ERROR))?;
            (&mut self.player_data[player_index]).num_cards = if num_cards == 0 { None } else { Some(num_cards)};
        }
        // Load the list of cards this player has
        while *tokenizer.peek().ok_or_else(|| String::from(STRING_ENDED_ERROR))? != '-' {
            self.learn_info_on_card(player_index, CardUtils::card_from_char(tokenizer.next().ok_or(String::from(STRING_ENDED_ERROR))?)?, true, true);
        }
        // advance past the '-'
        tokenizer.next();
        // Load the list of cards this player doesn't have
        {
            let mut next_char = *tokenizer.peek().ok_or(String::from(STRING_ENDED_ERROR))?;
            while next_char != '-' && next_char != '.' {
                self.learn_info_on_card(player_index, CardUtils::card_from_char(tokenizer.next().ok_or(String::from(STRING_ENDED_ERROR))?)?, false, true);
                next_char = *tokenizer.peek().ok_or(String::from(STRING_ENDED_ERROR))?;
            }
        }
        // Load the list of clauses as long as it's not done
        while tokenizer.next().ok_or(String::from(STRING_ENDED_ERROR))? != '.' {
            let mut clause = HashSet::new();
            let mut next_char = *tokenizer.peek().ok_or(String::from(STRING_ENDED_ERROR))?;
            while next_char != '-' && next_char != '.' {
                clause.insert(CardUtils::card_from_char(tokenizer.next().ok_or(String::from(STRING_ENDED_ERROR))?)?);
                next_char = *tokenizer.peek().ok_or(String::from(STRING_ENDED_ERROR))?;
            }
            if !clause.is_empty() {
                self.learn_has_one_of_cards(player_index, &clause);
            }
        }
        
        Ok(())
    }

    pub fn learn_info_on_card(self: &mut ClueEngine, player_index: usize, card: Card, has_card: bool, update_engine: bool) -> CardSet {
        let mut changed_cards = HashSet::new();
        let update_mode = UpdateEngineMode::from(update_engine);
        self.learn_info_on_card_internal(player_index, card, has_card, update_mode, &mut changed_cards);
        return changed_cards;
    }

    fn learn_info_on_card_internal(self: &mut ClueEngine, player_index: usize, card: Card, has_card: bool, update_engine: UpdateEngineMode, changed_cards: &mut CardSet) {
        {
            let player = &mut self.player_data[player_index];
            if has_card {
                player.has_cards.insert(card);
            }
            else {
                player.not_has_cards.insert(card);
            }
            if update_engine != UpdateEngineMode::None {
                changed_cards.insert(card);
                self.examine_clauses(player_index, Some(card), changed_cards);
            }
        }
        if update_engine == UpdateEngineMode::All  {
            self.check_solution(Some(card), changed_cards);
        }

        if update_engine != UpdateEngineMode::None {
            if has_card && self.player_data[player_index].is_solution_player {
                // We know we have no other cards in this category.
                for other_card in CardUtils::cards_of_type(CardUtils::card_type(card)) {
                    if other_card != card {
                        self.learn_info_on_card_internal(player_index, other_card, false, update_engine, changed_cards);
                    }
                }
            }
        }
    }

    // Requires that all cards be assigned
    fn is_consistent_after_all_cards_assigned(self: &mut ClueEngine) -> bool {
        let mut cards_seen = CardSet::new();
        for player in self.player_data.iter() {
            for &card in player.has_cards.iter() {
                if player.not_has_cards.contains(&card) {
                    // has card and doesn't have card
                    return false;
                }
                if !cards_seen.insert(card) {
                    // Already seen this card in someone else's cards, so not consistent
                    return false;
                }
            }
            if player.has_cards.len() != player.num_cards.unwrap() as usize {
                // wrong number of cards
                return false;
            }
            for clause in player.possible_cards.iter() {
                if !clause.intersection(&player.has_cards).any(|_| true) {
                    // This clause is not satisfied
                    return false;
                }
            }
        }
        return true;
    }

    pub fn learn_has_one_of_cards(self: &mut ClueEngine, player_index: usize, cards: &CardSet) -> CardSet {
        let mut changed_cards = HashSet::new();
        self.learn_has_one_of_cards_internal(player_index, cards, &mut changed_cards);
        return changed_cards;
    }

    fn learn_has_one_of_cards_internal(self: &mut ClueEngine, player_index: usize, cards: &CardSet, changed_cards: &mut CardSet) {
        let mut clause_helpful = true;
        let mut new_clause = HashSet::new();
        for card in cards.iter() {
            let has_card = self.player_data[player_index].has_card(*card);
            match has_card {
                Some(true) => {
                    // We already know player has one of these cards, so this
                    // clause is worthless.
                    clause_helpful = false;
                },
                Some(false) => {
                    // We know player doesn't have this card, so don't add this card
                    // to the new clause.
                },
                None => {
                    // Don't know; add it to the new clause
                    new_clause.insert(*card);
                }
            }
        }
        if clause_helpful && !new_clause.is_empty() {
            if new_clause.len() == 1 {
                // We have learned player has this card!
                let new_card = *new_clause.iter().next().unwrap();
                self.learn_info_on_card_internal(player_index, new_card, true, UpdateEngineMode::All, changed_cards);
            } else {
                self.player_data[player_index].possible_cards.push(new_clause);
            }
            self.examine_clauses(player_index, None, changed_cards);
        }
    }

    pub fn learn_suggest(self: &mut ClueEngine, suggesting_player_index: usize, card1: Card, card2: Card, card3: Card, refuting_player_index: Option<usize>, card_shown: Option<Card>) -> CardSet {
        let mut changed_cards = HashSet::new();
        self.learn_suggest_internal(suggesting_player_index, card1, card2, card3, refuting_player_index, card_shown, &mut changed_cards);
        return changed_cards;
    }

    fn learn_suggest_internal(self: &mut ClueEngine, suggesting_player_index: usize, card1: Card, card2: Card, card3: Card, refuting_player_index: Option<usize>, card_shown: Option<Card>, changed_cards: &mut CardSet) {
        let mut current_player_index = suggesting_player_index + 1;
        if current_player_index == self.number_of_real_players() as usize {
            current_player_index = 0;
        }
        loop {
            if refuting_player_index == Some(current_player_index) {
                if let Some(real_card) = card_shown {
                    self.learn_info_on_card_internal(current_player_index, real_card, true, UpdateEngineMode::All, changed_cards);
                } else {
                    let possible_cards = HashSet::from_iter(vec![card1, card2, card3].iter().map(|x| *x));
                    self.learn_has_one_of_cards_internal(current_player_index, &possible_cards, changed_cards);
                }
                self.check_solution(None, changed_cards);
                return;
            } else if current_player_index == suggesting_player_index {
                // No one can refute this.  We're done.
                self.check_solution(None, changed_cards);
                return;
            } else {
                self.learn_info_on_card_internal(current_player_index, card1, false, UpdateEngineMode::Minimal, changed_cards);
                self.learn_info_on_card_internal(current_player_index, card2, false, UpdateEngineMode::Minimal, changed_cards);
                self.learn_info_on_card_internal(current_player_index, card3, false, UpdateEngineMode::Minimal, changed_cards);
                current_player_index += 1;
                if current_player_index == self.number_of_real_players() as usize {
                    current_player_index = 0;
                }
            }
        }
    }

    fn examine_clauses(self: &mut ClueEngine, player_index: usize, card: Option<Card>, changed_cards: &mut CardSet) {
        self.player_data[player_index].eliminate_extraneous_clauses();
        if let Some(real_card) = card {
            let player = &mut self.player_data[player_index];
            // Iterate over all the clauses, but since we might be removing
            // things from the Vec, keep track of the current index manually.
            let mut i: usize = 0;
            while i < player.possible_cards.len() {
                let clause = &mut player.possible_cards[i];
                let mut skip_increment = false;
                if clause.contains(&real_card) {
                    if player.has_cards.contains(&real_card) {
                        // We have this card, so this clause is done
                        player.possible_cards.remove(i);
                        // adjust loop counter
                        skip_increment = true;
                    }
                    else if player.not_has_cards.contains(&real_card) {
                        clause.remove(&real_card);
                        if clause.len() == 1 {
                            // We have this card!
                            let have_card = clause.iter().next().unwrap();
                            player.has_cards.insert(*have_card);
                            changed_cards.insert(*have_card);
                            player.possible_cards.remove(i);
                            // adjust loop counter
                            skip_increment = true;
                        }
                    }
                }
                if !skip_increment {
                    i += 1;
                }
            }
        }
        if let Some(number_of_cards) = self.player_data[player_index].num_cards {
            if number_of_cards == self.player_data[player_index].has_cards.len() as u8 {
                // All cards are accounted for.
                for other_card in CardUtils::all_cards() {
                    if self.player_data[player_index].has_card(other_card) == None {
                        self.learn_info_on_card(player_index, other_card, false, true);
                    }
                }
            }
            else if self.player_data[player_index].has_cards.len() + self.player_data[player_index].possible_cards.len() > (number_of_cards as usize) {
                // We may be able to figure out something
                let num_accounted_for = number_of_cards as isize - self.player_data[player_index].has_cards.len() as isize;
                let card_in_any_clause: &CardSet = &self.player_data[player_index].possible_cards.iter().fold(
                    HashSet::new(),
                    |mut set, v| {set.extend(v.iter()); set});
                for test_card in card_in_any_clause {
                    // See if we could have this card, by contradiction.
                    // Assume we don't have this card.  Remove it from
                    // all clauses.
                    let new_clauses = Self::remove_card_from_clauses(&self.player_data[player_index].possible_cards, *test_card);
                    // See if it's possible to satisfy the rest of the clauses with one fewer card.
                    let is_possible = Self::can_satisfy(&new_clauses, num_accounted_for - 1);
                    if !is_possible {
                        // We found a contradiction if we don't have this card,
                        // so we must have this card.
                        self.learn_info_on_card_internal(player_index, *test_card, true, UpdateEngineMode::All, changed_cards);
                    }
                }
            }
        }
    }

    pub fn transpose_clauses(possible_cards: &Vec<CardSet>) -> HashMap<Card, HashSet<usize>> {
        let mut transposed_clauses: HashMap<Card, HashSet<usize>> = HashMap::new();
        for i in 0..possible_cards.len() {
            let clause = &possible_cards[i];
            for card in clause.iter() {
                if let Some(existing_clauses) = transposed_clauses.get_mut(card) {
                    existing_clauses.insert(i);
                }
                else {
                    let mut new_hash_set = HashSet::new();
                    new_hash_set.insert(i);
                    transposed_clauses.insert(*card, new_hash_set);
                }
            }
        }
        return transposed_clauses;
    }

    pub fn remove_card_from_clauses(clauses: &Vec<CardSet>, card: Card) -> Vec<CardSet> {
        let mut new_clauses = vec!();
        new_clauses.reserve(clauses.len());
        for clause in clauses {
            let mut new_clause = clause.clone();
            new_clause.remove(&card);
            new_clauses.push(new_clause);
        }
        return new_clauses;
    }

    // Returns whether there's a set of choices that can satisfy all these clauses,
    // given we can only use up to num_accounted_for cards.
    fn can_satisfy(clauses: &Vec<CardSet>, num_unaccounted_for: isize) -> bool {
        if clauses.len() == 0 {
            return true;
        }
        if num_unaccounted_for <= 0 {
            return false;
        }
        // If there are any empty clauses we have a contradiction already.
        let smallest_clause = clauses.iter().min_by_key(|x| x.len()).unwrap();
        if smallest_clause.len() == 0 {
            return false;
        }
        // See if there's any way we can satisfy these
        // Try one card at a time
        let card_clauses = ClueEngine::transpose_clauses(clauses);
        for test_card in smallest_clause {
            // First, remove all clauses containing this card.
            let new_clauses = ClueEngine::remove_clauses_with_indices(clauses, card_clauses.get(test_card).unwrap());
            // See if it's possible to satisfy the rest of the clauses with one fewer card.
            if ClueEngine::can_satisfy(&new_clauses, num_unaccounted_for - 1) {
                return true;
            }
        }
        return false;
    }

    pub fn remove_clauses_with_indices(clauses: &Vec<CardSet>, indices_to_remove: &HashSet<usize>) -> Vec<CardSet> {
        let mut new_clauses = vec!();
        for i in 0..clauses.len() {
            if !indices_to_remove.contains(&i) {
                new_clauses.push(clauses[i].clone());
            }
        }
        return new_clauses;
    }

    // Check if any cards are the solution, and also if any clauses are in common.
    fn check_solution(self: &mut Self, card: Option<Card>, changed_cards: &mut CardSet) {
        if let Some(real_card) = card {
            self.check_for_all_players_but_one_dont_have_this_card(real_card, changed_cards);
        }

        for card_type in CardUtils::all_card_types() {
            let all_cards = CardUtils::cards_of_type(*card_type).collect::<Vec<Card>>();
            let mut solution_card: Option<Card> = None;
            let mut is_solution = true;
            for test_card in all_cards.iter() {
                // See if anyone has this card
                let card_owned = self.player_data.iter().any(|player| player.has_card(*test_card) == Some(true));
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
                    // (if this happened, we're inconsistent already, just move on)
                    if all_cards.iter().all(|c| !self.solution_player().has_cards.contains(c)) {
                        self.solution_player_mut().has_cards.insert(solution);
                        changed_cards.insert(solution);
                    }
                }
            }
        }

        // Finally, see if any people share clauses in common.
        self.check_for_overlapping_clauses(changed_cards);
    }

    fn check_for_overlapping_clauses(self: &mut Self, changed_cards: &mut CardSet) {
        let mut clause_hash: HashMap<String, Vec<usize>> = HashMap::new();
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
            if clause.len() <= players.len() {
                let affected_people: HashSet<usize> = HashSet::from_iter(players.iter().map(|x| *x));
                for card in clause.chars().map(|ch| CardUtils::card_from_char(ch).unwrap()) {
                    changed_cards.insert(card);
                }
                for idx in 0..(self.number_of_real_players() + 1) {
                    if !affected_people.contains(&idx) {
                        for card in clause.chars().map(|ch| CardUtils::card_from_char(ch).unwrap()) {
                            if self.player_data[idx as usize].has_card(card) != Some(false) {
                                self.learn_info_on_card_internal(idx as usize, card, false, UpdateEngineMode::Minimal, changed_cards);
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_for_all_players_but_one_dont_have_this_card(self: &mut Self, card: Card, changed_cards: &mut CardSet) {
        let mut someone_has_card = false;
        let mut number_who_dont_have_card = 0;
        let mut player_who_might_have_card = None;
        // - Check also for all cards except one in a category are
        // accounted for.
        for i in 0..self.player_data.len() {
            let player = &mut self.player_data[i];
            let has_card = player.has_card(card);
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
                    // We only look at this if there's only one person who could have this card.
                    player_who_might_have_card = Some(i);
                }
            }
        }
        if !someone_has_card && number_who_dont_have_card == self.number_of_real_players() {
            // Every player except one doesn't have this card, so we know the player has it.
            self.learn_info_on_card_internal(player_who_might_have_card.unwrap(), card, true, UpdateEngineMode::Minimal, changed_cards);
        }
        else if someone_has_card {
            // Someone has this card, so no one else does. (including solution)
            for i in 0..self.player_data.len() {
                let player = &self.player_data[i];
                if player.has_card(card) == None {
                    self.learn_info_on_card_internal(i, card, false, UpdateEngineMode::Minimal, changed_cards);
                }
            }
        }
    }

    pub fn do_simulation(self: &Self) -> (SimulationData, i32) {
        const SIMULATION_IN_PARALLEL: bool = true;
        const NUM_SIMULATIONS_TO_SPLIT: i32 = 1000;
        let mut simulation_data = SimulationData::new();
        self.initialize_simulation_data(&mut simulation_data);
        if self.player_data.iter().any(|player| player.num_cards == None) {
            // Can't do simulations if we don't know how many cards everyone has
            return (simulation_data, 0);
        }
        // Find a solution to simulate.
        // FFV - this iteration could be more generalized
        let mut solution_possibilities: HashMap<CardType, Vec<Card>> = HashMap::new();
        let solution_cards = &self.player_data[self.number_of_real_players()].has_cards;
        let not_solution_cards = &self.player_data[self.number_of_real_players()].not_has_cards;
        for card_type in CardUtils::all_card_types() {
            let mut already_found_solution_iter = solution_cards.iter().filter(|&card| CardUtils::card_type(*card) == *card_type);
            let already_found_solution = already_found_solution_iter.next();
            if already_found_solution != None {
                // We know what the solution is for this card already
                solution_possibilities.insert(*card_type, vec![*(already_found_solution.unwrap())]);
            }
            else {
                // Take all possible cards, except for the ones we know aren't
                // solutions
                let all_possible_cards = CardUtils::cards_of_type(*card_type).collect::<HashSet<Card>>();
                solution_possibilities.insert(*card_type, all_possible_cards.iter().filter_map(|&card| if not_solution_cards.contains(&card) {None } else {Some(card)}).collect());
            }
        }
        let number_of_solutions = solution_possibilities.values().map(|cards| cards.len() as i32).product::<i32>();
        let iterations_per_solution = Self::NUM_SIMULATIONS / number_of_solutions;
        let mut solution_engines: Vec<(ClueEngine, CardSet, i32)> = vec![];
        for card1 in solution_possibilities.get(&CardType::Suspect).unwrap() {
            for card2 in solution_possibilities.get(&CardType::Weapon).unwrap() {
                for card3 in solution_possibilities.get(&CardType::Room).unwrap() {
                    let mut engine_copy = self.clone();
                    // To avoid solution biasing, we need to gather the available_cards before we put in the solution.
                    // Otherwise see the test test_simulation_monty_hall_no_player0
                    // In that case, ProfessorPlum only has two possibilities, and once we pick it (or something else)
                    // for the solution it automatically goes to the other player.
                    // But we should be throwing out a lot of those simulations.
                    let mut available_cards: CardSet = CardUtils::all_cards().collect();
                    for player in engine_copy.player_data.iter() {
                        for has_card in player.has_cards.iter() {
                            available_cards.remove(has_card);
                        }
                    }
                    available_cards.remove(card1);
                    available_cards.remove(card2);
                    available_cards.remove(card3);

                    // Call the internal versions to avoid a few allocations
                    let mut ignored_changed_cards = CardSet::new();
                    engine_copy.learn_info_on_card_internal(engine_copy.number_of_real_players(), *card1, true, UpdateEngineMode::All, &mut ignored_changed_cards);
                    engine_copy.learn_info_on_card_internal(engine_copy.number_of_real_players(), *card2, true, UpdateEngineMode::All, &mut ignored_changed_cards);
                    engine_copy.learn_info_on_card_internal(engine_copy.number_of_real_players(), *card3, true, UpdateEngineMode::All, &mut ignored_changed_cards);
                    if SIMULATION_IN_PARALLEL {
                        // Don't split on just cards, because of there are only a few solution possibilities
                        // we won't get good parallelism.
                        let mut temp_iterations_per_solution = iterations_per_solution;
                        while temp_iterations_per_solution > 0 {
                            solution_engines.push((engine_copy.clone(), available_cards.clone(), min(NUM_SIMULATIONS_TO_SPLIT, temp_iterations_per_solution)));
                            temp_iterations_per_solution -= NUM_SIMULATIONS_TO_SPLIT;
                        }
                    }
                    else {
                        solution_engines.push((engine_copy, available_cards, iterations_per_solution));
                    }
                }
            }
        }

        let simulations_per_iteration: i32 = solution_engines.iter().map(|data| data.2).sum();
        let total_number_of_simulations;
        if SIMULATION_IN_PARALLEL {
            let mut iterations = 0;
            const MAX_ITERATIONS: i32 = 100;
            while iterations < MAX_ITERATIONS && simulation_data.get(&Card::ProfessorPlum).unwrap().iter().sum::<usize>() < 1000 {
                iterations += 1;
                let results: Vec<SimulationData> = solution_engines.par_iter().map(|solution_data| {
                    let mut local_simulation_data = HashMap::new();
                    self.initialize_simulation_data(&mut local_simulation_data);
                    
                    let engine = &solution_data.0;
                    let available_cards = &solution_data.1;
                    let iterations = solution_data.2;
                    Self::gather_simulation_data(&mut local_simulation_data, &engine, available_cards, iterations);
                    local_simulation_data
                }).collect();
                for result in results {
                    Self::merge_into(&mut simulation_data, &result);
                }
            }
            total_number_of_simulations = iterations * simulations_per_iteration;
        }
        else {
            let mut iterations = 0;
            const MAX_ITERATIONS: i32 = 100;
            while iterations < MAX_ITERATIONS && simulation_data.get(&Card::ProfessorPlum).unwrap().iter().sum::<usize>() < 1000 {
                iterations += 1;
                for (engine, available_cards, iterations) in &solution_engines {
                    Self::gather_simulation_data(&mut simulation_data, &engine, &available_cards, *iterations);
                }
            }
            total_number_of_simulations = iterations * simulations_per_iteration;
        }

        return (simulation_data, total_number_of_simulations);
    }

    fn gather_simulation_data(simulation_data: &mut SimulationData, engine: &ClueEngine, available_cards: &CardSet, iterations: i32) {
        for _ in 0..iterations {
            let mut temp_engine = engine.clone();
            if ClueEngine::do_one_simulation(&mut temp_engine, available_cards) {
                // Results were consistent, so count them
                for player_index in 0..temp_engine.player_data.len() {
                    for card in temp_engine.player_data[player_index].has_cards.iter() {
                        simulation_data.get_mut(card).unwrap()[player_index] += 1;
                    }
                }
            }
        }
    }

    fn merge_into(target: &mut SimulationData, source: &SimulationData) {
        for (card, counts) in source {
            let target_counts = target.get_mut(card).unwrap();
            for i in 0..counts.len() {
                target_counts[i] += counts[i];
            }
        }
    }

    // Returns whether the simulation is consistent
    fn do_one_simulation(engine: &mut ClueEngine, available_cards: &CardSet) -> bool {
        const USE_UNBIASED_ALGORITHM: bool = true;
        let mut unused_cards = CardSet::new();
        if USE_UNBIASED_ALGORITHM {
            let mut temp_available_cards = available_cards.iter().collect::<Vec<&Card>>();
            // Assign all values randomly.
            for player_index in 0..engine.number_of_real_players() {
                let player = &engine.player_data[player_index];
                let num_cards_needed = player.num_cards.unwrap() as usize - player.has_cards.len();
                // If there are not enough cards available, we're
                // inconsistent.
                if temp_available_cards.len() < num_cards_needed {
                    return false;
                }
                else {
                    for _ in 0..num_cards_needed {
                        let index = (rand::random::<f32>() * temp_available_cards.len() as f32).floor() as usize;
                        let card_to_add = temp_available_cards.remove(index);
                        // see if we're going to be inconsistent and exit early
                        if engine.player_data[player_index].not_has_cards.contains(card_to_add) {
                            return false;
                        }
                        engine.learn_info_on_card_internal(player_index, *card_to_add, true, UpdateEngineMode::None, &mut unused_cards);
                    }
                }
            }
        }
        else {
            let mut temp_available_cards = available_cards.clone();
            for player_index in 0..engine.number_of_real_players() {
                let player = &engine.player_data[player_index];
                let num_cards_needed = player.num_cards.unwrap() as usize - player.has_cards.len();
                let mut player_cards_available = temp_available_cards.difference(&player.not_has_cards).map(|&card| card).collect::<Vec<Card>>();
                // If there are not enough cards available, we're
                // inconsistent.
                if player_cards_available.len() < num_cards_needed {
                    return false;
                }
                else {
                    for _ in 0..num_cards_needed {
                        let index = (rand::random::<f32>() * player_cards_available.len() as f32).floor() as usize;
                        let card_to_add = player_cards_available.remove(index);
                        temp_available_cards.remove(&card_to_add);
                        engine.learn_info_on_card_internal(player_index, card_to_add, true, UpdateEngineMode::None, &mut unused_cards);
                    }
                }
            }
        }
        // All players assigned.  Check consistency.
        return engine.is_consistent_after_all_cards_assigned();
    }

    fn initialize_simulation_data(self: &Self, data: &mut SimulationData) {
        for card in CardUtils::all_cards() {
            let zeros = (0..(self.player_data.len())).map(|_| 0).collect();
            data.insert(card, zeros);
        }
    }

    pub fn is_consistent(self: &Self) -> bool {
        return self.player_data.iter().all(|player|
             !player.has_cards.intersection(&player.not_has_cards).any(|_| true));
    }

    pub fn who_has_card(self: &Self, card: Card) -> HashSet<usize> {
        let mut possible_owners = HashSet::new();
        for i in 0..(self.number_of_real_players() + 1) {
            match self.player_data[i].has_card(card) {
                Some(true) => {
                    possible_owners.clear();
                    possible_owners.insert(i);
                    return possible_owners;
                },
                None => possible_owners.insert(i),
                _ => false
            };
        }
        return possible_owners;
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    fn make_card_set(cards: Vec<Card>) -> CardSet {
        return HashSet::from_iter(cards.iter().map(|x| *x));
    }

    fn make_usize_set(set: Vec<usize>) -> HashSet<usize> {
        return HashSet::from_iter(set.iter().map(|x| *x));
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
            assert_eq!(ch, CardUtils::char_from_card(CardUtils::card_from_char(ch).unwrap()));
        }
    }

    #[test]
    fn test_card_from_char_on_char_below_a_panics() {
        if let Ok(_) = CardUtils::card_from_char('0') {
            panic!("Should not parse!");
        }
    }
    #[test]
    fn test_card_from_char_on_char_above_u_panics() {
        if let Ok(_) = CardUtils::card_from_char('V') {
            panic!("Should not parse!");
        }
    }

    #[test]
    fn test_card_from_char() {
        assert_eq!(Card::ProfessorPlum, CardUtils::card_from_char('A').unwrap());
        assert_eq!(Card::ColonelMustard, CardUtils::card_from_char('B').unwrap());
        assert_eq!(Card::MrsPeacock, CardUtils::card_from_char('F').unwrap());
        assert_eq!(Card::Knife, CardUtils::card_from_char('G').unwrap());
        assert_eq!(Card::Wrench, CardUtils::card_from_char('L').unwrap());
        assert_eq!(Card::Hall, CardUtils::card_from_char('M').unwrap());
        assert_eq!(Card::BilliardRoom, CardUtils::card_from_char('U').unwrap());
    }

    #[test]
    fn test_card_set_to_sorted_string() {
        assert_eq!("ABC", CardUtils::card_set_to_sorted_string(&vec![Card::ColonelMustard, Card::ProfessorPlum, Card::MrGreen].into_iter().collect()));
        assert_eq!("", CardUtils::card_set_to_sorted_string(&HashSet::new()));
        assert_eq!("CLU", CardUtils::card_set_to_sorted_string(&vec![Card::BilliardRoom, Card::Wrench, Card::MrGreen].into_iter().collect()));
    }

    #[test]
    fn test_eliminate_extraneous_clauses_empty() {
        let mut clauses: Vec<CardSet> = vec![];
        PlayerData::eliminate_extraneous_clauses_possible_cards(&mut clauses);
        assert!(clauses.is_empty());
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
    fn test_transpose_clauses() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Library, Card::Wrench, Card::Conservatory]),
            make_card_set(vec![Card::Conservatory, Card::Wrench]),
            make_card_set(vec![Card::Library, Card::Hall])];
 
        let transposed = ClueEngine::transpose_clauses(&clauses);

        assert_eq!(6, transposed.len());
        assert_eq!(&make_usize_set(vec![0]), transposed.get(&Card::ProfessorPlum).unwrap());
        assert_eq!(&make_usize_set(vec![0]), transposed.get(&Card::MsWhite).unwrap());
        assert_eq!(&make_usize_set(vec![1, 3]), transposed.get(&Card::Library).unwrap());
        assert_eq!(&make_usize_set(vec![1, 2]), transposed.get(&Card::Wrench).unwrap());
        assert_eq!(&make_usize_set(vec![1, 2]), transposed.get(&Card::Conservatory).unwrap());
        assert_eq!(&make_usize_set(vec![3]), transposed.get(&Card::Hall).unwrap());
    }

    #[test]
    fn test_remove_card_from_clauses() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::Library, Card::Wrench, Card::Conservatory]),
            make_card_set(vec![Card::Conservatory, Card::Wrench]),
            make_card_set(vec![Card::Library, Card::Hall]),
            make_card_set(vec![Card::Wrench])];
 
        let removed = ClueEngine::remove_card_from_clauses(&clauses, Card::Wrench);

        assert_eq!(5, removed.len());
        assert_eq!(make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]), removed[0]);
        assert_eq!(make_card_set(vec![Card::Library, Card::Conservatory]), removed[1]);
        assert_eq!(make_card_set(vec![Card::Conservatory]), removed[2]);
        assert_eq!(make_card_set(vec![Card::Library, Card::Hall]), removed[3]);
        assert_eq!(make_card_set(vec![]), removed[4]);
    }

    #[test]
    fn test_can_satisfy_one_card() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library]),
            make_card_set(vec![Card::ProfessorPlum])];
        assert_eq!(true, ClueEngine::can_satisfy(&clauses, 1));
    }

    #[test]
    fn test_can_satisfy_no_cards() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library]),
            make_card_set(vec![Card::ProfessorPlum])];
        assert_eq!(false, ClueEngine::can_satisfy(&clauses, 0));
    }

    #[test]
    fn test_can_satisfy_extra_cards() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library]),
            make_card_set(vec![Card::ProfessorPlum])];
        assert_eq!(true, ClueEngine::can_satisfy(&clauses, 2));
    }
    
    #[test]
    fn test_can_satisfy_two_cards() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library]),
            make_card_set(vec![Card::Hall])];
        assert_eq!(true, ClueEngine::can_satisfy(&clauses, 2));
    }

    #[test]
    fn test_can_satisfy_needs_two_cards_but_only_room_for_one() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library]),
            make_card_set(vec![Card::Hall])];
        assert_eq!(false, ClueEngine::can_satisfy(&clauses, 1));
    }

    #[test]
    fn test_can_satisfy_needs_one_card_if_careful() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite, Card::Hall]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library, Card::Ballroom, Card::Hall]),
            make_card_set(vec![Card::Hall])];
        assert_eq!(true, ClueEngine::can_satisfy(&clauses, 1));
    }

    #[test]
    fn test_can_satisfy_empty_vec() {
        let clauses: Vec<CardSet> = vec![];
        assert_eq!(true, ClueEngine::can_satisfy(&clauses, 1));
    }

    #[test]
    fn test_can_satisfy_empty_clause() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite, Card::Hall]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library, Card::Ballroom, Card::Hall]),
            make_card_set(vec![])];
        assert_eq!(false, ClueEngine::can_satisfy(&clauses, 2));
    }

    #[test]
    fn test_remove_clauses_with_indices_empty() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite, Card::Hall]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library, Card::Ballroom, Card::Hall]),
            make_card_set(vec![Card::Hall])];
        let new_clauses = ClueEngine::remove_clauses_with_indices(&clauses, &HashSet::new());

        assert_eq!(clauses, new_clauses);
    }

    #[test]
    fn test_remove_clauses_with_indices_multiple() {
        let clauses: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite, Card::Hall]),
            make_card_set(vec![Card::ProfessorPlum, Card::Library, Card::Ballroom, Card::Hall]),
            make_card_set(vec![Card::Hall]),
            make_card_set(vec![Card::Library, Card::MsWhite, Card::Candlestick])];
        let new_clauses = ClueEngine::remove_clauses_with_indices(&clauses, &make_usize_set(vec![1, 3]));

        let expected: Vec<CardSet> = vec![
            make_card_set(vec![Card::ProfessorPlum, Card::MsWhite, Card::Hall]),
            make_card_set(vec![Card::Hall])];
        assert_eq!(expected, new_clauses);
    }

    #[test]
    fn test_merge_into_single_key() {
        let mut target = SimulationData::new();
        target.insert(Card::ProfessorPlum, vec![1,2,3]);
        let mut source = SimulationData::new();
        source.insert(Card::ProfessorPlum, vec![7,8,9]);

        ClueEngine::merge_into(&mut target, &source);

        assert_eq!(target[&Card::ProfessorPlum], vec![8,10,12]);
    }

    #[test]
    fn test_merge_into_multiple_keys() {
        let mut target = SimulationData::new();
        target.insert(Card::ProfessorPlum, vec![1,2,3]);
        target.insert(Card::Knife, vec![0,1,0]);
        target.insert(Card::Hall, vec![10,2,0]);
        let mut source = SimulationData::new();
        source.insert(Card::Hall, vec![3,7,1]);
        source.insert(Card::Knife, vec![4,2,0]);
        source.insert(Card::ProfessorPlum, vec![6,3,8]);

        ClueEngine::merge_into(&mut target, &source);

        assert_eq!(target[&Card::ProfessorPlum], vec![7,5,11]);
        assert_eq!(target[&Card::Knife], vec![4,3,0]);
        assert_eq!(target[&Card::Hall], vec![13,9,1]);
    }
}
