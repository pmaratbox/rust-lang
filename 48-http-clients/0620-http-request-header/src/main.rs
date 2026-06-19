use std::thread;

fn main() {
    // In-process server on an ephemeral loopback port (never printed).
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port();
    thread::spawn(move || {
        for req in server.incoming_requests() {
            if req.url() == "/token" {
                // Echo the X-Token request header back in the body.
                let tok = req
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv("X-Token"))
                    .map(|h| h.value.to_string())
                    .unwrap_or_default();
                req.respond(tiny_http::Response::from_string(tok)).unwrap();
            } else {
                req.respond(tiny_http::Response::from_string("").with_status_code(404))
                    .unwrap();
            }
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(format!("{}/token", base))
        .header("X-Token", "secret")
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}", body);
}
