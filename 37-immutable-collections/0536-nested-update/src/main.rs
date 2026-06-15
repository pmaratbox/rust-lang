use im::HashMap;

fn main() {
    // Persistent map { "user": { "age": 30 } }.
    let inner: HashMap<&str, i32> = HashMap::unit("age", 30);
    let outer: HashMap<&str, HashMap<&str, i32>> = HashMap::unit("user", inner);

    // Immutable nested update: `update` returns a NEW map; `outer` is unchanged.
    // Build the new inner map with the updated age, then update the outer key.
    let new_inner = outer["user"].update("age", 31);
    let updated = outer.update("user", new_inner);

    // New nested age, then the original nested age (still 30).
    println!("{}", updated["user"]["age"]);
    println!("{}", outer["user"]["age"]);
}
