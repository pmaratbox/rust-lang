use std::thread;

fn main() {
    // In-process server on an ephemeral loopback port (never printed).
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port();
    thread::spawn(move || {
        for req in server.incoming_requests() {
            let url = req.url().to_string();
            let resp = if url.starts_with("/greet") {
                let name = url.split("name=").nth(1).unwrap_or("");
                tiny_http::Response::from_string(format!("hi {}", name))
            } else {
                tiny_http::Response::from_string("").with_status_code(404)
            };
            req.respond(resp).unwrap();
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(format!("{}/greet", base))
        .query(&[("name", "Bob")])
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}", body);
}
