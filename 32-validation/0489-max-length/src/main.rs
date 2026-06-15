use validator::Validate;

#[derive(Validate)]
struct Model {
    #[validate(length(max = 5))]
    code: String,
}

fn main() {
    let m = Model {
        code: "ABCDEFG".into(),
    };
    match m.validate() {
        Ok(()) => println!("ok"),
        Err(e) => {
            let mut fields: Vec<String> =
                e.field_errors().keys().map(|k| k.to_string()).collect();
            fields.sort();
            println!("{}", fields.join("\n"));
        }
    }
}
