// reqwest::blocking client + tiny_http in-process server on 127.0.0.1:0.
// GET /hello returns 200; we read and print the integer status code.
use std::thread;

fn main() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port(); // never PRINT the port
    thread::spawn(move || {
        for req in server.incoming_requests() {
            let resp = if req.url() == "/hello" {
                tiny_http::Response::from_string("hello world")
            } else {
                tiny_http::Response::from_string("").with_status_code(404)
            };
            req.respond(resp).unwrap();
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let c = reqwest::blocking::Client::new();
    let status = c.get(format!("{}/hello", base)).send().unwrap().status().as_u16();
    println!("{}", status);
}
