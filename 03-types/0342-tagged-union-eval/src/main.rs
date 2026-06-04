enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
    }
}

fn main() {
    let expr = Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)));
    println!("{}", eval(&expr));
}
