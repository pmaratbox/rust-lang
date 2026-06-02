use std::collections::BTreeSet;

fn main() {
    let a: BTreeSet<i32> = [1, 2, 3].into_iter().collect();
    let b: BTreeSet<i32> = [2, 3, 4].into_iter().collect();
    let union: Vec<String> = a.union(&b).map(|x| x.to_string()).collect();
    let inter: Vec<String> = a.intersection(&b).map(|x| x.to_string()).collect();
    println!("{}", union.join(" "));
    println!("{}", inter.join(" "));
}
