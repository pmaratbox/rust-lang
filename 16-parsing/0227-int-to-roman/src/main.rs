fn main() {
    let mut n = 14;
    let table = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for (value, sym) in table {
        while n >= value {
            out.push_str(sym);
            n -= value;
        }
    }
    println!("{}", out);
}
