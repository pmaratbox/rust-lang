use serde::Serialize;

#[derive(Serialize)]
struct Sample {
    active: bool,
    count: i32,
    label: String,
}

fn main() {
    let sample = Sample {
        active: true,
        count: 5,
        label: "x".into(),
    };
    println!("{}", serde_json::to_string(&sample).unwrap());
}
