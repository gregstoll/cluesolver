use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{collections::HashSet, collections::HashMap, iter::Peekable, str::Chars};
use std::iter::FromIterator;

pub type CardSet = HashSet<Card>;

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

pub struct ClueEngine {
    pub player_data: Vec<PlayerData>,
}

impl ClueEngine {
    pub fn new(number_of_players: u8) -> ClueEngine {
        let mut player_datas: Vec<PlayerData> = vec!();
        for i in 0..(number_of_players + 1) {
            let player_data = PlayerData::new(Some(ClueEngine::number_of_player_cards(i, number_of_players)), i == number_of_players);
            player_datas.push(player_data);
        }
        let clue_engine = ClueEngine { player_data: player_datas };
        return clue_engine;
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
        let mut num_cards = 18 / num_players; // Integer division
        let leftovers = 18 % num_players;
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

    pub fn load_from_string(s: &str) -> ClueEngine {
        let mut tokenizer = Tokenizer::new(s);
        let number_of_players = tokenizer.next_digit();
        let mut clue_engine = ClueEngine::new(number_of_players);
        for i in 0..(number_of_players+1) {
            clue_engine.load_player_from_string(i as usize, &mut tokenizer);
        }
        return clue_engine;
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
    fn load_player_from_string(self: &mut ClueEngine, player_index: usize, tokenizer: &mut Tokenizer) {
        {
            let num_cards = tokenizer.next_digit() as u8;
            (&mut self.player_data[player_index]).num_cards = if num_cards == 0 { None } else { Some(num_cards)};
        }
        // Load the list of cards this player has
        while *tokenizer.peek().unwrap() != '-' {
            self.learn_info_on_card(player_index, CardUtils::card_from_char(tokenizer.next().unwrap()), true, true);
        }
        // advance past the '-'
        tokenizer.next();
        // Load the list of cards this player doesn't have
        {
            let mut next_char = *tokenizer.peek().unwrap();
            while next_char != '-' && next_char != '.' {
                self.learn_info_on_card(player_index, CardUtils::card_from_char(tokenizer.next().unwrap()), false, true);
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
                self.learn_has_one_of_cards(player_index, &clause);
            }
        }
    }

    pub fn learn_info_on_card(self: &mut ClueEngine, player_index: usize, card: Card, has_card: bool, update_engine: bool) -> CardSet {
        let mut changed_cards = HashSet::new();
        {
            let player = &mut self.player_data[player_index];
            if has_card {
                player.has_cards.insert(card);
            }
            else {
                player.not_has_cards.insert(card);
            }
            changed_cards.insert(card);
            let other_changed_cards = self.examine_clauses(player_index, Some(card));
            changed_cards.extend(other_changed_cards.iter());
        }
        if update_engine {
            changed_cards.extend(self.check_solution(Some(card)).iter());
        }
        if has_card && self.player_data[player_index].is_solution_player {
            // We know we have no other cards in this category.
            for other_card in CardUtils::cards_of_type(CardUtils::card_type(card)) {
                if other_card != card {
                    changed_cards.extend(self.learn_info_on_card(player_index, other_card, false, true).iter());
                }
            }
        }

        return changed_cards;
    }

    pub fn learn_has_one_of_cards(self: &mut ClueEngine, player_index: usize, cards: &CardSet) -> CardSet {
        let mut clause_helpful = true;
        let mut changed_cards = HashSet::new();
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
                let other_changed_cards = self.learn_info_on_card(player_index, new_card, true, true);
                changed_cards.extend(other_changed_cards.iter());
            } else {
                self.player_data[player_index].possible_cards.push(new_clause);
            }
            let updated_cards = self.examine_clauses(player_index, None);
            changed_cards.extend(updated_cards.iter());
        }
        return changed_cards;
    }

    pub fn learn_suggest(self: &mut ClueEngine, suggesting_player_index: usize, card1: Card, card2: Card, card3: Card, refuting_player_index: Option<usize>, card_shown: Option<Card>) {
        let mut current_player_index = suggesting_player_index + 1;
        if current_player_index == self.number_of_real_players() as usize {
            current_player_index = 0;
        }
        loop {
            if refuting_player_index == Some(current_player_index) {
                if let Some(real_card) = card_shown {
                    self.learn_info_on_card(current_player_index, real_card, true, true);
                } else {
                    let possible_cards = HashSet::from_iter(vec![card1, card2, card3].iter().map(|x| *x));
                    self.learn_has_one_of_cards(current_player_index, &possible_cards);
                }
                self.check_solution(None);
                return;
            } else if current_player_index == suggesting_player_index {
                // No one can refute this.  We're done.
                self.check_solution(None);
                return;
            } else {
                self.learn_info_on_card(current_player_index, card1, false, false);
                self.learn_info_on_card(current_player_index, card2, false, false);
                self.learn_info_on_card(current_player_index, card3, false, false);
                current_player_index += 1;
                if current_player_index == self.number_of_real_players() as usize {
                    current_player_index = 0;
                }
            }
        }
    }

    fn examine_clauses(self: &mut ClueEngine, player_index: usize, card: Option<Card>) -> CardSet{
        self.player_data[player_index].eliminate_extraneous_clauses();
        let mut changed_cards = HashSet::new();
        if let Some(real_card) = card {
            let player = &mut self.player_data[player_index];
            // TODO - reexamine this and simplify after it's working
            let mut possible_cards_copy = player.possible_cards.clone();
            //for clause in possible_cards_copy {
            let mut adjustment = 0;
            for i in 0..possible_cards_copy.len() {
                let clause = &mut possible_cards_copy[i];
                if clause.contains(&real_card) {
                    if player.has_cards.contains(&real_card) {
                        // We have this card, so this clause is done
                        player.possible_cards.remove(i - adjustment);
                        adjustment += 1;
                    }
                    else if player.not_has_cards.contains(&real_card) {
                        (&mut player.possible_cards[i - adjustment]).remove(&real_card);
                        clause.remove(&real_card);
                        if clause.len() == 1 {
                            // We have this card!
                            let have_card = clause.iter().next().unwrap();
                            player.has_cards.insert(*have_card);
                            changed_cards.insert(*have_card);
                            player.possible_cards.remove(i - adjustment);
                            adjustment += 1;
                        }
                    }
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
                let num_accounted_for = number_of_cards as usize - self.player_data[player_index].has_cards.len();
                let card_clauses = Self::transpose_clauses(&self.player_data[player_index].possible_cards);
                //TODO - this is weird. why do we transpose_clauses and then basically ignore the result?
                for test_card in card_clauses.keys() {
                    // See if we could have this card, by contradiction.
                    // Assume we don't have this card.  Remove it from
                    // all clauses.
                    let new_clauses = Self::remove_card_from_clauses(&self.player_data[player_index].possible_cards, *test_card);
                    // If there are any empty clauses we have a contradiction already.
                    let is_possible;
                    if new_clauses.iter().any(|clause| clause.len() == 0) {
                        is_possible = false;
                    } else {
                        // See if it's possible to satisfy the rest of the clauses with one fewer card.
                        is_possible = Self::can_satisfy(&new_clauses, num_accounted_for - 1);
                    }
                    if !is_possible {
                        // We found a contradiction if we don't have this card,
                        // so we must have this card.
                        let other_changed_cards = self.learn_info_on_card(player_index, *test_card, true, true);
                        changed_cards.extend(other_changed_cards.iter());
                    }
                }
            }
        }
        return changed_cards;
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
    pub fn can_satisfy(clauses: &Vec<CardSet>, num_unaccounted_for: usize) -> bool {
        if clauses.len() == 0 {
            return true;
        }
        if num_unaccounted_for == 0 {
            return false;
        }
        // See if there's any way we can satisfy these
        // Try one card at a time
        // TODO - optimize this? this might be a hotspot
        //  - should be able to use the smallest clause or something
        let card_clauses = ClueEngine::transpose_clauses(clauses);
        for test_card in card_clauses.keys() {
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
                let other_changed_cards = self.learn_info_on_card(player_who_might_have_card.unwrap(), real_card, true, false);
                changed_cards.extend(other_changed_cards.iter());
            }
            else if someone_has_card {
                // Someone has this card, so no one else does. (including solution)
                for i in 0..self.player_data.len() {
                    let player = &self.player_data[i];
                    if player.has_card(real_card) == None {
                        let other_changed_cards = self.learn_info_on_card(i, real_card, false, false);
                        changed_cards.extend(other_changed_cards.iter());
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
                for card in clause.chars().map(|ch| CardUtils::card_from_char(ch)) {
                    changed_cards.insert(card);
                }
                for idx in 0..(self.number_of_real_players() + 1) {
                    if !affected_people.contains(&idx) {
                        for card in clause.chars().map(|ch| CardUtils::card_from_char(ch)) {
                            if self.player_data[idx as usize].has_card(card) != Some(false) {
                                let other_changed_cards = self.learn_info_on_card(idx as usize, card, false, false);
                                changed_cards.extend(other_changed_cards.iter());
                            }
                        }
                    }
                }
            }
        }
        return changed_cards;
    }

    pub fn is_consistent(self: &Self) -> bool {
        for player in self.player_data.iter() {
            if player.has_cards.intersection(&player.not_has_cards).any(|_x| true) {
                return false;
            }
        }
        return true;
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
            assert_eq!(ch, CardUtils::char_from_card(CardUtils::card_from_char(ch)));
        }
    }

    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_below_a_panics() {
        let _ch = CardUtils::card_from_char('0');
    }
    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_above_u_panics() {
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

}