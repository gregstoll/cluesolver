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

cgi::cgi_main! { |request: cgi::Request| {
    let possible_query = request.uri().query();
    if possible_query == None {
        // TODO - use try_cgi_main! which returns a Result<> instead?
        return error("No query string?");
    }
    let query = possible_query.unwrap();
    let query_parts: HashMap<String, String> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
    let action_result = query_parts.get("action");
    if action_result == None {
        return error("Internal error - no action specified!");
    }
    let action = action_result.unwrap();
    // Valid actions are 'new', 'whoOwns', 'suggestion', 'fullInfo', 'simulate' ('accusation' in the future?)
    // TODO - make this an enum or something
    if action != "new" && action != "whoOwns" && action != "suggestion" && action != "fullInfo" && action != "simulate" {
        return error(&format!("Internal error - invalid action \"{}\"!", action));
    }
    if action != "new" && !query_parts.contains_key("sess") {
        return error("Internal error - missing sess!");
    }
    let mut engine: clueengine::ClueEngine;
    if action == "new" {
        let players_result = query_parts.get("players");
        if players_result == None {
            return error("Internal error - action new without players!");
        }
        let players_str = players_result.unwrap();
        let players_int_result = players_str.parse::<u8>();
        if let Err(_) = players_int_result {
            return error(&format!("Internal error - couldn't parse players string to u8: {}", players_str));
        }
        let num_players = players_int_result.unwrap();
        engine = clueengine::ClueEngine::new(num_players);
        for i in 0..num_players {
            let key = format!("numCards{}", i);
            // TODO - need a way to specify these to ClueEngine::new()
            //engine.player_data[i]
        }
    }
    else {
        let engine_result = clueengine::ClueEngine::load_from_string(query_parts.get("sess").unwrap());
        if let Err(_) = engine_result {
            return error(&format!("Internal error - invalid session string '{}'!", query_parts.get("sess").unwrap()) );
        }
        engine = engine_result.unwrap();
    }
    //TODO
    return success(&json::object!{debug: format!("action is {}", query_parts.get("action").unwrap())});
} }