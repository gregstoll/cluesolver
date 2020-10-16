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
    let possible_query = request.uri().query();
    if possible_query == None {
        // TODO - use try_cgi_main! which returns a Result<> instead?
        return Err(String::from("No query string?"));
    }
    let query = possible_query.unwrap();
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
    let mut engine: clueengine::ClueEngine;
    if action == "new" {
        let players_str = query_parts.get("players").ok_or(String::from("Internal error - action new without players!"))?;
        let num_players = players_str.parse::<u8>()
            .map_err(|x| format!("Internal error - couldn't parse players string to u8: {}", players_str))?;
        let mut number_of_cards: Vec<u8> = vec!();
        for i in 0..num_players {
            let key = format!("numCards{}", i);
            let number_of_cards_str = query_parts.get(&key)
                .ok_or(format!("Internal error - action new missing key numCards{}!", i))?;
            let real_number = number_of_cards_str.parse::<u8>()
                .map_err(|_| format!("Internal error - action new can't parse numCards{} value \"{}\"!", i, number_of_cards_str))?;
            number_of_cards.push(real_number);
        }
        engine = clueengine::ClueEngine::new(num_players, Some(&number_of_cards))?;
    }
    else {
        engine = clueengine::ClueEngine::load_from_string(query_parts.get("sess").unwrap())
            .map_err(|x|format!("Internal error - invalid session string '{}': error \"{}\"", query_parts.get("sess").unwrap(), x))?;
    }
    //TODO
    return Ok(json::object!{debug: format!("action is {}", query_parts.get("action").unwrap())});
}

cgi::cgi_main! { |request: cgi::Request| {
    let result = process_request(&request);
    match result {
        Ok(val) => success(&val),
        Err(err) => error(&err)
    }
} }