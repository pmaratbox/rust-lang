fn is_even(n: i32) -> bool {
    if n == 0 {
        return true;
    }
    is_odd(n - 1)
}

fn is_odd(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    is_even(n - 1)
}

fn main() {
    for n in [4, 3] {
        println!("{}", if is_even(n) { "even" } else { "odd" });
    }
}
