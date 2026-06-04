fn main() {
    let houses = [2, 7, 9, 3, 1];
    let (mut skip, mut rob) = (0u64, 0u64);
    for &v in &houses {
        let new_rob = skip + v as u64;
        skip = skip.max(rob);
        rob = new_rob;
    }
    println!("{}", skip.max(rob));
}
