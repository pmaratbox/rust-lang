# 0467 — Field rename

Serialize a struct with `serde` and `serde_json`, mapping a code field to a different JSON key with serde's `#[serde(rename = "...")]` attribute. The struct's `name` field is emitted under the JSON key `full_name`, producing compact JSON `{"full_name":"alice"}`.

## Run

    cargo run
