fn main() {
    let s = "5";
    let left = format!("{s:>3}");
    let right = format!("{s:<3}");
    println!("{left}|{right}");
}
