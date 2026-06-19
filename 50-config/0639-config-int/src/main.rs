use config::{Config, Environment, File};

fn main() {
    let c = Config::builder()
        .set_default("missing", "fallback")
        .unwrap()
        .add_source(File::with_name("config.json"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Read the top-level integer `retries`.
    println!("{}", c.get_int("retries").unwrap());
}
