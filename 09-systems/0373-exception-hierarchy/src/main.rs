// Model an error hierarchy with an enum: a specific subtype variant matched
// by a base-category handler.
enum AppError {
    Base(SpecificError),
}

enum SpecificError {
    NotFound,
}

fn fail() -> Result<(), AppError> {
    Err(AppError::Base(SpecificError::NotFound))
}

fn main() {
    match fail() {
        Ok(()) => println!("ok"),
        Err(AppError::Base(_)) => println!("caught base"),
    }
}
