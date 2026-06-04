fn main() {
    let haystack = "ababab";
    let needle = "ab";
    let count = haystack.matches(needle).count();
    println!("{}", count);
}
