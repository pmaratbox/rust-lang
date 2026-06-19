use std::thread;

fn main() {
    // In-process server on an ephemeral loopback port (never printed).
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port();
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
    let client = reqwest::blocking::Client::new();
    let body = client.get(format!("{}/hello", base)).send().unwrap().text().unwrap();
    println!("{}", body);
}
