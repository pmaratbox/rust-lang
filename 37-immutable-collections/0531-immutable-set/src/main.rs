use im::HashSet;

fn main() {
    // Persistent set {1, 2, 3} from the `im` crate.
    let a: HashSet<i32> = im::hashset![1, 2, 3];

    // `update` returns a NEW set with 4 added; `a` is left unchanged
    // thanks to structural sharing.
    let b = a.update(4);

    // New set's size, then the original's size.
    println!("{}", b.len());
    println!("{}", a.len());
}
