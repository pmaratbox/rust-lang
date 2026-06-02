struct MathUtil;

impl MathUtil {
    fn square(n: i32) -> i32 {
        n * n
    }
}

fn main() {
    println!("{}", MathUtil::square(6));
}
