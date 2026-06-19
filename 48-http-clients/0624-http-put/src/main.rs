// reqwest::blocking client + tiny_http in-process server (127.0.0.1:0).
use std::thread;

fn main() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port(); // never PRINT the port
    thread::spawn(move || {
        for req in server.incoming_requests() {
            let (method, url) = (req.method().to_string(), req.url().to_string());
            let resp = if url == "/item" && method == "PUT" {
                tiny_http::Response::from_string("updated")
            } else {
                tiny_http::Response::from_string("").with_status_code(404)
            };
            req.respond(resp).unwrap();
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let c = reqwest::blocking::Client::new();
    println!("{}", c.put(format!("{}/item", base)).send().unwrap().text().unwrap());
}
