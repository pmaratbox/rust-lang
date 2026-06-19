// reqwest::blocking client + tiny_http in-process server on 127.0.0.1:0.
// The server defines no matching route, so GET /missing returns 404;
// we read and print the integer status code.
use std::thread;

fn main() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let port = server.server_addr().to_ip().unwrap().port(); // never PRINT the port
    thread::spawn(move || {
        for req in server.incoming_requests() {
            // No route defined for this lesson -> everything is 404.
            let resp = tiny_http::Response::from_string("").with_status_code(404);
            req.respond(resp).unwrap();
        }
    });

    let base = format!("http://127.0.0.1:{}", port);
    let c = reqwest::blocking::Client::new();
    let status = c.get(format!("{}/missing", base)).send().unwrap().status().as_u16();
    println!("{}", status);
}
