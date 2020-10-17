extern crate cgi;
extern crate json;
extern crate url;
use std::collections::HashMap;

fn error(s: &str) -> cgi::Response {
    cgi::text_response(200, (json::object!{"errorStatus": 1, "errorText": s.clone()}).dump())
}

fn success(s: &json::JsonValue) -> cgi::Response {
    let mut result = s.clone();
    result["errorStatus"] = json::JsonValue::Number(0.into());
    return cgi::text_response(200, result.dump());
}

fn process_request(request: &cgi::Request) -> Result<json::JsonValue, String> {
    let query = request.uri().query().ok_or(String::from("Internal error - no query string?"))?;
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
        let card_str = query_parts.get("card").ok_or(String::from("Internal error - missing card!"))?;
        if card_str.len() != 1 {
            return Err(format!("Invalid card \"{}\"!", card_str));
        }
        let ch = card_str.chars().next().ok_or(format!("Invalid card char \"{}\"!", card_str))?;
        let card = clueengine::CardUtils::card_from_char(ch)?;
        let changed_cards = engine.learn_info_on_card(owner as usize, card, true, true);
        /*return Ok(json::object! {
            "clauseInfo":
            "session": engine.write_to_string(),
            "isConsistent": engine.is_consistent()
        });*/

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

cgi::cgi_main! { |request: cgi::Request| {
    let result = process_request(&request);
    match result {
        Ok(val) => success(&val),
        Err(err) => error(&err)
    }
} }