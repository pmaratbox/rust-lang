fn main() {
    let a = "abcd";
    let b = "cdab";
    let is_rotation = a.len() == b.len() && format!("{a}{a}").contains(b);
    println!("{}", if is_rotation { "yes" } else { "no" });
}
