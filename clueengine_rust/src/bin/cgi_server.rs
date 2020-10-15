extern crate cgi;

cgi::cgi_main! { |request: cgi::Request| {
    cgi::text_response(200, "Hello world")
} }