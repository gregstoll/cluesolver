extern crate cgi;
extern crate json;
extern crate url;
use std::collections::HashMap;

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
        let card_str = query_parts.get("card").ok_or(String::from("Internal error - missing card!"))?;
        let card = card_from_string(card_str).map_err(|_| format!("Internal error - invalid card \"{}\"", card_str))?;
        let changed_cards = engine.learn_info_on_card(owner as usize, card, true, true);
        return Ok(json::object! {
            "newInfo": get_info_from_changed_cards(&engine, &changed_cards),
            "clauseInfo": get_clause_info(&engine),
            "session": engine.write_to_string(),
            "isConsistent": engine.is_consistent()
        });

    }
    // TODO
    return Ok(json::object! {"debug": format!("action is {}", query_parts.get("action").unwrap())});
}

fn get_clause_info(engine: &clueengine::ClueEngine) -> json::JsonValue {
    let mut info = json::object!{};
    for i in 0..engine.player_data.len() {
        //let mut cur_info = vec![];
        for clause in engine.player_data[i].possible_cards.iter() {
            //TODO
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
        let result = process_query_string("action=new&players=5&numCards0=4&numCards1=4&numCards2=4&numCards3=3&numCards4=3");
        assert!(result.is_ok());
        let expected = json::parse("{\"session\": \"54-.4-.4-.3-.3-.3-.\"}").unwrap();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_whoOwns_no_sess_error() {
        //let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=0&card=ProfessorPlum");
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
    fn test_whoOwns_playerOwns() {
        let result = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=0&card=ProfessorPlum");
        assert!(result.is_ok());
        let expected = json::parse(r#"{"newInfo": [{"card": "ProfessorPlum", "status": 1, "owner": [0]}], "clauseInfo": {}, "session": "63A-.3-A.3-A.3-A.3-A.3-A.3-A.", "isConsistent": true}"#).unwrap();
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_whoOwns_solutionOwns() {
        let result_wrapped = process_query_string("sess=63-.3-.3-.3-.3-.3-.3-.&action=whoOwns&owner=6&card=ProfessorPlum");
        assert!(result_wrapped.is_ok());
        let mut result = result_wrapped.unwrap();

        normalize_new_info(&mut result);
        let expected = json::parse(r#"{"newInfo": [{"card": "MsWhite", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MrGreen", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "ColonelMustard", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MrsPeacock", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "MissScarlet", "status": 1, "owner": [0, 1, 2, 3, 4, 5]}, {"card": "ProfessorPlum", "status": 2, "owner": [6]}], "clauseInfo": {}, "session": "63-A.3-A.3-A.3-A.3-A.3-A.3A-BCDEF.", "isConsistent": true}"#).unwrap();
        assert_eq!(expected, result.clone(), "got {}", json::stringify(result));
    }

    fn normalize_new_info(val: &mut json::JsonValue) {
        if val.has_key("newInfo") {
            let newInfo = &val["newInfo"];
            let mut newNewInfo = json::object![];
            //TODO - finish

            //for newInfo.entries()
        }
    }
}