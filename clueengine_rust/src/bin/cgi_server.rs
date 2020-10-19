extern crate cgi;
extern crate json;
extern crate url;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn error(s: &str) -> cgi::Response {
    cgi::binary_response(200, "application/json", (json::object!{"errorStatus": 1, "errorText": s.clone()}).dump().as_bytes().to_vec())
}

fn success(s: &json::JsonValue) -> cgi::Response {
    let mut result = s.clone();
    result["errorStatus"] = json::JsonValue::Number(0.into());
    return cgi::binary_response(200, "application/json", result.dump().as_bytes().to_vec());
}

fn process_request(request: &cgi::Request) -> Result<json::JsonValue, String> {
    let query = request.uri().query().ok_or(String::from("Internal error - no query string?"))?;
    return process_query_string(query);
}

fn process_query_string(query: &str) -> Result<json::JsonValue, String> {
    let query_parts: HashMap<String, String> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
    let action = query_parts.get("action").ok_or(String::from("Internal error - no action specified!"))?;
    // Valid actions are 'new', 'whoOwns', 'suggestion', 'fullInfo', 'simulate' ('accusation' in the future?)
    // TODO - make this an enum or something
    if action != "new" && action != "whoOwns" && action != "suggestion" && action != "fullInfo" && action != "simulate" {
        return Err(format!("Internal error - invalid action \"{}\"!", action));
    }
    if action != "new" && !query_parts.contains_key("sess") {
        return Err(String::from("Internal error - missing sess!"));
    }
    if action == "new" {
        let players_str = query_parts.get("players").ok_or(String::from("Internal error - action new without players!"))?;
        let num_players = players_str.parse::<u8>()
            .map_err(|_| format!("Internal error - couldn't parse players string to u8: {}", players_str))?;
        let mut number_of_cards: Vec<u8> = vec!();
        for i in 0..num_players {
            let key = format!("numCards{}", i);
            let number_of_cards_str = query_parts.get(&key)
                .ok_or(format!("Internal error - action new missing key numCards{}!", i))?;
            let real_number = number_of_cards_str.parse::<u8>()
                .map_err(|_| format!("Internal error - action new can't parse numCards{} value \"{}\"!", i, number_of_cards_str))?;
            number_of_cards.push(real_number);
        }
        let engine = clueengine::ClueEngine::new(num_players, Some(&number_of_cards))?;
        return Ok(json::object! {"session": engine.write_to_string()});
    }

    let mut engine = clueengine::ClueEngine::load_from_string(query_parts.get("sess").unwrap())
        .map_err(|x|format!("Internal error - invalid session string '{}': error \"{}\"", query_parts.get("sess").unwrap(), x))?;

    if action == "whoOwns" {
        let owner_str = query_parts.get("owner").ok_or(String::from("Internal error - missing owner!"))?;
        let owner = owner_str.parse::<u8>().map_err(|_| String::from("Internal error - bad owner!"))?;
        if owner as usize >= engine.player_data.len() {
            return Err(String::from("Internal error - owner out of range!"));
        }
        let card = card_from_query_parts(&query_parts, "card")?;
        let changed_cards = engine.learn_info_on_card(owner as usize, card, true, true);
        return Ok(json::object! {
            "newInfo": get_info_from_changed_cards(&engine, &changed_cards),
            "clauseInfo": get_clause_info(&engine),
            "session": engine.write_to_string(),
            "isConsistent": engine.is_consistent()
        });
    }
    if action == "suggestion" {
        let suggesting_player_str = query_parts.get("suggestingPlayer").ok_or("Internal error - no suggestingPlayer")?;
        let suggesting_player = suggesting_player_str.parse::<u8>().map_err(|_| String::from("Internal error - suggestingPlayer can't parse"))?;
        if suggesting_player as usize >= engine.number_of_real_players() {
            return Err(String::from("Internal error - suggesting_player out of range!"));
        }
        let card1 = card_from_query_parts(&query_parts, "card1")?;
        let card2 = card_from_query_parts(&query_parts, "card2")?;
        let card3 = card_from_query_parts(&query_parts, "card3")?;
        let refuting_player_str = query_parts.get("refutingPlayer").ok_or("Internal error - no refutingPlayer")?;
        let refuting_player_number = refuting_player_str.parse::<i16>().map_err(|_| format!("Internal error - couldn't parse refutingPlayer \"{}\"", refuting_player_str))?;
        if refuting_player_number < -1 || refuting_player_number >= engine.number_of_real_players() as i16 {
            return Err(String::from("Internal error - refuting player out of range!"));
        }
        let refuting_player = if refuting_player_number == -1 { None } else { Some(refuting_player_number as usize)};
        let refuting_card = optional_card_from_query_parts(&query_parts, "refutingCard")?;
        let changed_cards = engine.learn_suggest(suggesting_player as usize, card1, card2, card3, refuting_player, refuting_card);
        return Ok(json::object! {
            "newInfo": get_info_from_changed_cards(&engine, &changed_cards),
            "clauseInfo": get_clause_info(&engine),
            "session": engine.write_to_string(),
            "isConsistent": engine.is_consistent()
        });
    }
    if action == "fullInfo" {
        let all_cards = HashSet::from_iter(clueengine::CardUtils::all_cards());
        //TODO
        //let number_of_cards = engine.player_data.iter().map
        return Ok(json::object! {
            "newInfo": get_info_from_changed_cards(&engine, &all_cards),
            "clauseInfo": get_clause_info(&engine),
            "session": engine.write_to_string(),
            "numPlayers": engine.number_of_real_players(),
            "numCards": [], // TODO
            "isConsistent": engine.is_consistent()
        });
    }
    // TODO
    return Ok(json::object! {"debug": format!("action is {}", query_parts.get("action").unwrap())});
}

fn get_clause_info(engine: &clueengine::ClueEngine) -> json::JsonValue {
    let mut info = json::JsonValue::new_object();
    for i in 0..engine.player_data.len() {
        let mut cur_info = json::JsonValue::new_array();
        for clause in engine.player_data[i].possible_cards.iter() {
            cur_info.push(clause.iter().map(|x| format!("{:?}", *x)).collect::<Vec<String>>()).unwrap();
        }
        if cur_info.len() > 0 {
            info[i.to_string()] = cur_info;
        }
    }
    info
}

fn get_info_from_changed_cards(engine: &clueengine::ClueEngine, changed_cards: &clueengine::CardSet) -> json::JsonValue {
    let mut info = json::array![];
    for card in changed_cards.iter() {
        let possible_owners = engine.who_has_card(*card);
        let status = if possible_owners.len() == 1 {
            if *possible_owners.iter().next().unwrap() == engine.number_of_real_players() {
                2  // Solution
            } else {
                1
            }
        } else {
            if possible_owners.contains(&engine.number_of_real_players()) {
                0
            } else {
                1
            }
        };
        let mut owners_sorted = possible_owners.iter().map(|x| *x).collect::<Vec<usize>>();
        owners_sorted.sort();
        info.push(json::object!{
            "card": format!("{:?}", *card),
            "status": status,
            "owner": json::from(owners_sorted)
        }).unwrap();
    }
    info
}

fn card_from_query_parts(query_parts: &HashMap<String, String>, key: &str) -> Result<clueengine::Card, String> {
    let card_str = query_parts.get(key).ok_or(format!("Internal error - missing card with key {}!", key))?;
    return card_from_string(card_str).map_err(|_| format!("Internal error - bad card string {} for key {}", card_str, key));
}

fn optional_card_from_query_parts(query_parts: &HashMap<String, String>, key: &str) -> Result<Option<clueengine::Card>, String> {
    let card_str = query_parts.get(key).ok_or(format!("Internal error - missing card with key {}!", key))?;
    if card_str == "None" {
        return Ok(None);
    }
    return card_from_string(card_str).map(|card| Some(card)).map_err(|_| format!("Internal error - bad card string {} for key {}", card_str, key));
}
fn card_from_string(s: &str) -> Result<clueengine::Card, ()> {
    // This is inefficient, but oh well
    for card in clueengine::CardUtils::all_cards() {
        if format!("{:?}", card) == s {
            return Ok(card);
        }
    }
    Err(())
}

cgi::cgi_main! { |request: cgi::Request| {
    let result = process_request(&request);
    match result {
        Ok(val) => success(&val),
        Err(err) => error(&err)
    }
} }


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn test_new_no_players_error() {
        let result = process_query_string("action=new");
        assert!(result.is_err());
    }

    #[test]
    fn test_new_no_numcards_error() {
        let result = process_query_string("action=new&players=3&numCards0=6&numCards1=6");
        assert!(result.is_err());
    }

    #[test]
    fn test_new_cards_match() {
        assert_querystring_results_match(
            "action=new&players=5&numCards0=4&numCards1=4&numCards2=4&numCards3=3&numCards4=3",
            "{\"session\": \"54-.4-.4-.3-.3-.3-.\"}");
    }

    #[test]
    fn test_whoOwns_no_sess_error() {
        let result = process_query_string("action=whoOwns&owner=0&card=ProfessorPlum");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_no_owner_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&card=ProfessorPlum");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_owner_negative_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=-1&card=ProfessorPlum");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_owner_toobig_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=7&card=ProfessorPlum");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_owner_none_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=None&card=ProfessorPlum");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_no_card_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=0");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_card_invalid_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=0&card=NotACard");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_card_none_error() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=0&card=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_whoOwns_playerOwns() {
        assert_querystring_results_match(
            "sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=0&card=ProfessorPlum",
            r#"{"newInfo": [{"card": "ProfessorPlum", "status": 1, "owner": [0]}], "clauseInfo": {}, "session": "63A-.3-A.3-A.3-A.3-A.3-A.3-A.", "isConsistent": true}"#);
    }

    #[test]
    fn test_whoOwns_solutionOwns() {
        assert_querystring_results_match(
            "sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=6&card=ProfessorPlum",
            r#"{"newInfo": [{"card": "MsWhite", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MrGreen", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "ColonelMustard", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MrsPeacock", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MissScarlet", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "ProfessorPlum", "status": 2, "owner": [6]}], "clauseInfo": {}, "session": "63-A.3-A.3-A.3-A.3-A.3-A.3A-BCDEF.", "isConsistent": true}"#);
    }

    #[test]
    fn test_whoOwns_solutionOwns_with_clause_info() {
        assert_querystring_results_match(
            "sess=63--ABC.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=6&card=ProfessorPlum",
            r#"{"newInfo": [{"card": "MrsPeacock", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MissScarlet", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "ColonelMustard", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MsWhite", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "ProfessorPlum", "status": 2, "owner": [6]}, {"card": "MrGreen", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}], "clauseInfo": {"0": [["ColonelMustard", "MrGreen"]]}, "session": "63-A-BC.3-A.3-A.3-A.3-A.3-A.3A-BCDEF.", "isConsistent": true}"#);
    }

    #[test]
    fn test_suggestion_knownplayer_knowncard() {
        assert_querystring_results_match(
            "action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=4&refutingCard=Knife",
            r#"{"newInfo": [{"card": "Hall", "status": 0, "owner": [0, 1, 4, 5, 6]}, {"card": "Knife", "status": 1, "owner": [4]}, {"card": "ProfessorPlum", "status": 0, "owner": [0, 1, 4, 5, 6]}], "clauseInfo": {}, "session": "63-G.3-G.3-AGM.3-AGM.3G-.3-G.3-G.", "isConsistent": true}"#);
    }

    #[test]
    fn test_suggestion_notrefuted() {
        assert_querystring_results_match(
            "action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None",
            r#"{"newInfo": [{"card": "ProfessorPlum", "status": 0, "owner": [1, 6]}, {"card": "Knife", "status": 0, "owner": [1, 6]}, {"card": "Hall", "status": 0, "owner": [1, 6]}], "clauseInfo": {}, "session": "63-AGM.3-.3-AGM.3-AGM.3-AGM.3-AGM.3-.", "isConsistent": true}"#);
    }

    #[test]
    fn test_suggestion_suggestingPlayer_negative_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=-1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_suggestingPlayer_None_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=None&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_suggestingPlayer_missing_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card1_invalid_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=NotACard&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card1_none_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=None&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card1_missing_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card2=Knife&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card2_invalid_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=NotACard&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card2_none_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=None&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card2_missing_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card3=Hall&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card3_invalid_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=NotACard&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card3_none_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=None&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_card3_missing_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&refutingPlayer=-1&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_refutingPlayer_negative_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=-2&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_refutingPlayer_toobig_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=6&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_refutingPlayer_None_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=None&refutingCard=None");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_refutingCard_invalid_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=2&refutingCard=NotACard");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggestion_refutingCard_missing_error() {
        let result = process_query_string("action=suggestion&sess=63-.3-.3-.3-.3-.3-.3-.&suggestingPlayer=1&card1=ProfessorPlum&card2=Knife&card3=Hall&refutingPlayer=2");
        assert!(result.is_err());
    }

    #[test]
    fn test_normalize_new_info() {
        let mut result = json::parse(r#"{"newInfo": [{"card": "z", "status": 0, "owner": [0]}, {"card": "a", "status": 1, "owner": [1]}]}"#).unwrap();
        let expected= json::parse(r#"{"newInfo": [{"card": "a", "status": 1, "owner": [1]}, {"card": "z", "status": 0, "owner": [0]}]}"#).unwrap();
        normalize_new_info(&mut result);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_normalize_clause_info() {
        let mut result = json::parse(r#"{"clauseInfo": {"0": [["a", "b"], ["y", "x"]], "1": [["d", "c"]]}}"#).unwrap();
        let expected = json::parse(r#"{"clauseInfo": {"0": [["a", "b"], ["x", "y"]], "1": [["c", "d"]]}}"#).unwrap();
        normalize_clause_info(&mut result);
        assert_eq!(expected, result);
    }

    fn assert_querystring_results_match(query_string: &str, expected_result: &str) {
        let mut result = process_query_string(query_string).unwrap();
        let mut expected = json::parse(expected_result).unwrap();
        normalize(&mut result);
        normalize(&mut expected);
        assert_eq!(expected, result.clone(), "got {}", json::stringify(result));
    }

    fn normalize(val: &mut json::JsonValue) {
        normalize_new_info(val);
        normalize_clause_info(val);
    }

    // The order of this array doesn't matter, so sort them for testing purposes
    fn normalize_new_info(val: &mut json::JsonValue) {
        if val.has_key("newInfo") {
            let newInfo = &val["newInfo"];
            let mut newNewInfo = json::array![];
            let mut values = newInfo.members().map(|x| x.clone()).collect::<Vec<json::JsonValue>>();
            values.sort_by(|x, y| x["card"].as_str().unwrap().partial_cmp(y["card"].as_str().unwrap()).unwrap());
            for value in values {
                newNewInfo.push(value).unwrap();
            }
            val["newInfo"] = newNewInfo;
        }
    }

    // The order of the clauses doesn't matter, so sort them for testing purposes
    fn normalize_clause_info(val: &mut json::JsonValue) {
        if val.has_key("clauseInfo") {
            let clause_info = &mut val["clauseInfo"];
            // This is pretty ugly, got a little tired of fighting with the borrow checker
            let cloned_clause_info = clause_info.clone();
            let entries = cloned_clause_info.entries().collect::<Vec<(&str, &json::JsonValue)>>();
            for (key, array_of_clauses) in entries {
                let mut new_array_of_clauses = json::array![];
                for clause in array_of_clauses.members() {
                    let mut new_clause = clause.members().map(|x| x.clone()).collect::<Vec<json::JsonValue>>();
                    new_clause.sort_by(|x, y| x.as_str().unwrap().partial_cmp(y.as_str().unwrap()).unwrap());
                    new_array_of_clauses.push(new_clause).unwrap();
                }
                clause_info[key] = new_array_of_clauses;
            }
        }
    }
}