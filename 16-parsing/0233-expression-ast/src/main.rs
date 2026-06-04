enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn main() {
    // 1 + 2 * 3
    let ast = Expr::Add(
        Box::new(Expr::Num(1)),
        Box::new(Expr::Mul(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))),
    );
    println!("{}", eval(&ast));
}
