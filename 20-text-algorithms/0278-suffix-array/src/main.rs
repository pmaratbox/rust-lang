fn main() {
    let s = "banana";
    let mut sa: Vec<usize> = (0..s.len()).collect();
    sa.sort_by_key(|&i| &s[i..]);
    let out: Vec<String> = sa.iter().map(|x| x.to_string()).collect();
    println!("{}", out.join(" "));
}
