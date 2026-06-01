use std::env;

fn main() {
    let value = env::var("LESSON_ENV_VAR").unwrap_or_else(|_| "default".to_string());
    println!("value: {value}");
}
