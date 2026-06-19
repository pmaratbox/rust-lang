fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    let value = vec![1, 2, 3];
    let bytes = rmp_serde::to_vec(&value).unwrap();
    println!("{}", hexs(&bytes)); // 93010203
}
