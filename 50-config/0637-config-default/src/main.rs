use config::{Config, Environment, File};

fn main() {
    let c = Config::builder()
        .set_default("missing", "fallback")
        .unwrap()
        .add_source(File::with_name("config.json"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // `missing` is not in config.json, so the registered default wins.
    println!("{}", c.get_string("missing").unwrap());
}
