extern crate cgi;
extern crate json;
extern crate url;
use std::collections::HashMap;

fn error(s: &str) -> String {
    (json::object!{"errorStatus": 1, "errorText": s.clone()}).dump()
}

fn success(s: &json::JsonValue) -> String {
    let mut result = s.clone();
    //result.insert("errorStatus", 0);
    result["errorStatus"] = json::JsonValue::Number(0.into());
    return result.dump();
}

cgi::cgi_main! { |request: cgi::Request| {
    let possible_query = request.uri().query();
    if possible_query == None {
        // TODO - use try_cgi_main! which returns a Result<> instead?
        return cgi::text_response(500, error("No query string?"));
    }
    let query = possible_query.unwrap();
    let query_parts: HashMap<String, String> = url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();
    cgi::text_response(200, String::from("action is ") + query_parts.get("action").unwrap())
} }