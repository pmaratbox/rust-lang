use config::{Config, Environment, File};

fn main() {
    // Set the env var IN-PROCESS, before building, so the library merges it
    // with HIGHER priority than the file and overrides `name`.
    std::env::set_var("APP_NAME", "from-env");

    let c = Config::builder()
        .set_default("missing", "fallback")
        .unwrap()
        .add_source(File::with_name("config.json"))
        .add_source(Environment::with_prefix("APP")) // APP_NAME overrides `name`
        .build()
        .unwrap();

    println!("{}", c.get_string("name").unwrap());
}
