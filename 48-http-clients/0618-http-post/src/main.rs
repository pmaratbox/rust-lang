use std::io::Read;
use std::thread;

fn main() {
    // In-process server on an ephemeral loopback port (never printed).
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port();
    thread::spawn(move || {
        for mut req in server.incoming_requests() {
            if req.url() == "/echo" && req.method().to_string() == "POST" {
                // Echo the request body back verbatim.
                let mut body = String::new();
                req.as_reader().read_to_string(&mut body).unwrap();
                req.respond(tiny_http::Response::from_string(body)).unwrap();
            } else {
                req.respond(tiny_http::Response::from_string("").with_status_code(404))
                    .unwrap();
            }
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let client = reqwest::blocking::Client::new();
    let body = client
        .post(format!("{}/echo", base))
        .body("ping")
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}", body);
}
