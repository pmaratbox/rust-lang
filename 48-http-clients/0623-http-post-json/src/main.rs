// Rust — reqwest::blocking client + tiny_http in-process server (127.0.0.1:0).
use std::io::Read;
use std::thread;

fn main() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port(); // never PRINT the port
    thread::spawn(move || {
        for mut req in server.incoming_requests() {
            let (method, url) = (req.method().to_string(), req.url().to_string());
            if url == "/double" && method == "POST" {
                let mut b = String::new();
                req.as_reader().read_to_string(&mut b).unwrap();
                let x = serde_json::from_str::<serde_json::Value>(&b).unwrap()["x"]
                    .as_i64()
                    .unwrap();
                req.respond(tiny_http::Response::from_string(format!(
                    "{{\"doubled\":{}}}",
                    x * 2
                )))
                .unwrap();
            } else {
                req.respond(tiny_http::Response::from_string("").with_status_code(404))
                    .unwrap();
            }
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let c = reqwest::blocking::Client::new();
    let d: serde_json::Value = c
        .post(format!("{}/double", base))
        .json(&serde_json::json!({ "x": 5 }))
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("{}", d["doubled"]);
}
