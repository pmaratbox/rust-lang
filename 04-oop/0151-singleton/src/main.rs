use std::sync::OnceLock;

struct Config {
    name: String,
}

fn instance() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| Config {
        name: "config".to_string(),
    })
}

fn main() {
    let a = instance();
    let b = instance();
    let same = std::ptr::eq(a, b);
    println!("same: {}", if same { "yes" } else { "no" });
}
