use validator::{Validate, ValidationError};

fn has_digit(password: &str) -> Result<(), ValidationError> {
    if password.chars().any(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(ValidationError::new("no_digit"))
    }
}

#[derive(Validate)]
struct Model {
    #[validate(custom(function = "has_digit"))]
    password: String,
}

fn main() {
    let m = Model {
        password: "abcdef".into(),
    };
    match m.validate() {
        Ok(()) => println!("ok"),
        Err(e) => {
            let mut fields: Vec<String> = e.field_errors().keys().map(|k| k.to_string()).collect();
            fields.sort();
            println!("{}", fields.join("\n"));
        }
    }
}
