enum Bounce {
    More(Box<dyn FnOnce() -> Bounce>),
    Done(u64),
}

fn sum_step(n: u64, acc: u64) -> Bounce {
    if n == 0 {
        Bounce::Done(acc)
    } else {
        Bounce::More(Box::new(move || sum_step(n - 1, acc + n)))
    }
}

fn run(mut b: Bounce) -> u64 {
    loop {
        match b {
            Bounce::More(thunk) => b = thunk(),
            Bounce::Done(v) => return v,
        }
    }
}

fn main() {
    println!("{}", run(sum_step(100, 0)));
}
