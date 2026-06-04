#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.age.cmp(&other.age)
    }
}

fn main() {
    let mut people = vec![
        Person { name: "alice".to_string(), age: 30 },
        Person { name: "bob".to_string(), age: 25 },
    ];
    people.sort();
    let names: Vec<&str> = people.iter().map(|p| p.name.as_str()).collect();
    println!("{}", names.join(" "));
}
