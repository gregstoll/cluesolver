use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, FromPrimitive)]
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
 
pub struct PlayerData<'a> {
    clue_engine: ClueEngine<'a>,
}

pub struct ClueEngine<'a> {
    player_data: Vec<&'a mut PlayerData<'a>>,
}

impl<'a> ClueEngine<'a> {
    fn new(number_of_players: u8) -> ClueEngine<'a> {
        let mut player_datas: Vec<&'a mut PlayerData<'a>> = vec!();
        ClueEngine { player_data: player_datas }
    }

    fn card_from_char(ch: char) -> Card {
        let index = ch as u32 - 'A' as u32;
        return FromPrimitive::from_u32(index).unwrap()
    }

    fn char_from_card(card: Card) -> char {
        let index = card as u8 + 'A' as u8;
        return index as char;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_char_from_card() {
        assert_eq!(ClueEngine::char_from_card(Card::ProfessorPlum), 'A');
        assert_eq!(ClueEngine::char_from_card(Card::ColonelMustard), 'B');
        assert_eq!(ClueEngine::char_from_card(Card::MrsPeacock), 'F');
        assert_eq!(ClueEngine::char_from_card(Card::Knife), 'G');
        assert_eq!(ClueEngine::char_from_card(Card::Wrench), 'L');
        assert_eq!(ClueEngine::char_from_card(Card::Hall), 'M');
        assert_eq!(ClueEngine::char_from_card(Card::BilliardRoom), 'U');
        for i in ('A' as u8)..('V' as u8) {
            let ch = i as char;
            assert_eq!(ch, ClueEngine::char_from_card(ClueEngine::card_from_char(ch)));
        }
    }

    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_below_a__panics() {
        let ch = ClueEngine::card_from_char('0');
    }
    #[test]
    #[should_panic]
    fn test_card_from_char_on_char_above_u__panics() {
        let ch = ClueEngine::card_from_char('V');
    }

    #[test]
    fn test_card_from_char() {
        assert_eq!(ClueEngine::card_from_char('A'), Card::ProfessorPlum);
        assert_eq!(ClueEngine::card_from_char('B'), Card::ColonelMustard);
        assert_eq!(ClueEngine::card_from_char('F'), Card::MrsPeacock);
        assert_eq!(ClueEngine::card_from_char('G'), Card::Knife);
        assert_eq!(ClueEngine::card_from_char('L'), Card::Wrench);
        assert_eq!(ClueEngine::card_from_char('M'), Card::Hall);
        assert_eq!(ClueEngine::card_from_char('U'), Card::BilliardRoom);
    }
 
}
