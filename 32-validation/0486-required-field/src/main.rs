use validator::Validate;

// The schema requires both `name` and `age`. Optional fields model the
// "may be missing" case; the built-in `required` validator errors when the
// value is `None`, so a missing required field surfaces in `field_errors()`.
#[derive(Validate)]
struct Model {
    #[validate(required)]
    name: Option<String>,
    #[validate(required)]
    age: Option<i32>,
}

fn main() {
    // `name` is present, `age` is MISSING.
    let m = Model {
        name: Some("alice".into()),
        age: None,
    };

    match m.validate() {
        Ok(()) => println!("ok"),
        Err(e) => {
            let mut fields: Vec<String> = e
                .field_errors()
                .keys()
                .map(|k| k.to_lowercase())
                .collect();
            fields.sort();
            println!("{}", fields.join("\n"));
        }
    }
}
