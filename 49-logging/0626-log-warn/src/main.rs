use std::io::Write;
use std::sync::{Arc, Mutex};

// An in-memory writer shared with the tracing subscriber. Every log line the
// subscriber emits is appended here instead of going to the terminal, so the
// test can read the captured record back out.
#[derive(Clone)]
struct Buf(Arc<Mutex<Vec<u8>>>);

impl Write for Buf {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let buf = Buf(Arc::new(Mutex::new(Vec::new())));
    let w = buf.clone();

    // JSON formatter, no timestamp, writing into our in-memory buffer.
    let sub = tracing_subscriber::fmt()
        .json()
        .without_time()
        .with_max_level(tracing::Level::WARN)
        .with_writer(move || w.clone())
        .finish();

    // Emit a WARN record with the message `low disk`.
    tracing::subscriber::with_default(sub, || {
        tracing::warn!("low disk");
    });

    // Each captured line looks like:
    // {"level":"WARN","fields":{"message":"low disk"},"target":"..."}
    let text = String::from_utf8(buf.0.lock().unwrap().clone()).unwrap();
    for line in text.lines() {
        let d: serde_json::Value = serde_json::from_str(line).unwrap();
        let lvl = match d["level"].as_str().unwrap() {
            "INFO" => "info",
            "WARN" => "warn",
            "ERROR" => "error",
            "DEBUG" => "debug",
            o => o,
        };
        let f = d["fields"].as_object().unwrap();
        let msg = f["message"].as_str().unwrap();
        let mut keys: Vec<&String> = f.keys().filter(|k| *k != "message").collect();
        keys.sort();
        let mut out = format!("{}|{}", lvl, msg);
        for k in keys {
            let v = &f[k];
            let vs = if v.is_string() {
                v.as_str().unwrap().to_string()
            } else {
                v.to_string()
            };
            out.push_str(&format!("|{}={}", k, vs));
        }
        println!("{}", out);
    }
}
