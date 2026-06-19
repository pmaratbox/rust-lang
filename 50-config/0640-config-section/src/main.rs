use config::{Config, Environment, File};

fn main() {
    let c = Config::builder()
        .set_default("missing", "fallback")
        .unwrap()
        .add_source(File::with_name("config.json"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Read multiple keys from the `server` section and combine them as host:port.
    println!(
        "{}:{}",
        c.get_string("server.host").unwrap(),
        c.get_int("server.port").unwrap()
    );
}
