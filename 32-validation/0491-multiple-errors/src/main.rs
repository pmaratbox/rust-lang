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
        name: "al".into(),
        age: 200,
    };
    match m.validate() {
        Ok(()) => println!("ok"),
        Err(e) => {
            // field_errors() aggregates ALL failing fields (not fail-fast).
            let mut fields: Vec<String> =
                e.field_errors().keys().map(|k| k.to_string()).collect();
            fields.sort();
            println!("{}", fields.join("\n"));
        }
    }
}
