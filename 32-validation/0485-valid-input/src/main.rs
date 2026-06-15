use validator::Validate;

#[derive(Validate)]
struct Model {
    #[validate(length(min = 3))]
    name: String,
    #[validate(range(min = 0, max = 120))]
    age: i32,
}

fn main() {
    let m = Model {
        name: "alice".into(),
        age: 30,
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
