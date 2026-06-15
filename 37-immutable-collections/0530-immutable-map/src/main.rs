use im::HashMap;

fn keys_sorted(m: &HashMap<&str, i32>) -> String {
    let mut ks: Vec<&str> = m.keys().copied().collect();
    ks.sort();
    ks.join(" ")
}

fn main() {
    let a: HashMap<&str, i32> = HashMap::unit("a", 1);
    // `update` returns a NEW map; `a` stays unchanged.
    let b = a.update("b", 2);
    println!("{}", keys_sorted(&b));
    println!("{}", keys_sorted(&a));
}
