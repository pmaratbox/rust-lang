use im::Vector;

fn main() {
    let a: Vector<i32> = im::vector![1, 2, 3];
    // `update` returns a NEW vector with index 0 set to 99; `a` is untouched.
    let b = a.update(0, 99);
    println!("{}", b.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
