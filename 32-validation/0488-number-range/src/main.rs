use validator::Validate;

#[derive(Validate)]
struct Person {
    #[validate(length(min = 3))]
    name: String,
    #[validate(range(min = 0, max = 120))]
    age: i32,
}

fn main() {
    let person = Person {
        name: "alice".into(),
        age: 200,
    };

    match person.validate() {
        Ok(()) => println!("ok"),
        Err(e) => {
            let mut fields: Vec<String> =
                e.field_errors().keys().map(|k| k.to_lowercase()).collect();
            fields.sort();
            println!("{}", fields.join("\n"));
        }
    }
}
