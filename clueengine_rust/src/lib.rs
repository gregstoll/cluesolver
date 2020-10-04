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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2, 2);
    }
}
