#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Level {
    Info = 0,
    Warn = 1,
    Error = 2,
}

impl Level {
    fn name(self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

fn main() {
    let threshold = Level::Warn;
    let messages = [(Level::Info, "i"), (Level::Warn, "w"), (Level::Error, "e")];
    for (level, msg) in messages {
        if level >= threshold {
            println!("{}: {}", level.name(), msg);
        }
    }
}
