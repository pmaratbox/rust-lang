use im::HashSet;

fn main() {
    let a: HashSet<i32> = HashSet::unit(1).update(2).update(3);
    let b: HashSet<i32> = HashSet::unit(3).update(4).update(5);

    // `union` RETURNS A NEW set; `a` and `b` stay unchanged.
    let u = a.clone().union(b.clone());

    let mut elems: Vec<i32> = u.into_iter().collect();
    elems.sort();
    println!(
        "{}",
        elems
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
