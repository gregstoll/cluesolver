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
    let possible_action = query_parts.get("action");
    if possible_action == None {
        return error("Internal error - no action specified!");
    }
    let action = possible_action.unwrap();
    // Valid actions are 'new', 'whoOwns', 'suggestion', 'fullInfo', 'simulate' ('accusation' in the future?)
    // TODO - make this an enum or something
    if action != "new" && action != "whoOwns" && action != "suggestion" && action != "fullInfo" && action != "simulate" {
        return error(&format!("Internal error - invalid action \"{}\"!", action));
    }
    if action != "new" && !query_parts.contains_key("sess") {
        return error("Internal error - missing sess!");
    }
    return success(&json::object!{debug: format!("action is {}", query_parts.get("action").unwrap())});
} }