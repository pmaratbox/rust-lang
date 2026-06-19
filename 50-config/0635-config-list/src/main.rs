use config::{Config, Environment, File};

fn main() {
    let c = Config::builder()
        .set_default("missing", "fallback")
        .unwrap()
        .add_source(File::with_name("config.json"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let hosts = c
        .get_array("hosts")
        .unwrap()
        .into_iter()
        .map(|v| v.into_string().unwrap())
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", hosts);
}
