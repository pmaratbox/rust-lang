fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    let n: Option<i32> = None;
    println!("{}", hexs(&rmp_serde::to_vec(&n).unwrap())); // c0
}
