// reqwest::blocking client + tiny_http in-process server (127.0.0.1:0).
use std::thread;

fn main() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port(); // never PRINT the port
    thread::spawn(move || {
        for req in server.incoming_requests() {
            if req.url() == "/info" {
                let h = tiny_http::Header::from_bytes(&b"X-Count"[..], &b"7"[..]).unwrap();
                req.respond(tiny_http::Response::from_string("").with_header(h))
                    .unwrap();
            } else {
                req.respond(tiny_http::Response::from_string("").with_status_code(404))
                    .unwrap();
            }
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let c = reqwest::blocking::Client::new();
    let info = c.get(format!("{}/info", base)).send().unwrap();
    println!("{}", info.headers().get("X-Count").unwrap().to_str().unwrap());
}
