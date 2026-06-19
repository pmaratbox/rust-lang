// reqwest::blocking client + tiny_http in-process server (127.0.0.1:0).
use std::thread;

fn main() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port(); // never PRINT the port
    thread::spawn(move || {
        for req in server.incoming_requests() {
            let resp = if req.url() == "/user" {
                tiny_http::Response::from_string("{\"name\":\"Alice\",\"age\":30}")
            } else {
                tiny_http::Response::from_string("").with_status_code(404)
            };
            req.respond(resp).unwrap();
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let c = reqwest::blocking::Client::new();
    let u: serde_json::Value = c.get(format!("{}/user", base)).send().unwrap().json().unwrap();
    println!("{}", u["name"].as_str().unwrap()); // Alice
}
