#[derive(Clone)]
struct Inner {
    b: i32,
}

#[derive(Clone)]
struct Outer {
    a: Inner,
}

struct Lens<S, A> {
    get: fn(&S) -> A,
    set: fn(&S, A) -> S,
}

fn main() {
    let b_lens = Lens::<Outer, i32> {
        get: |s| s.a.b,
        set: |s, v| {
            let mut copy = s.clone();
            copy.a.b = v;
            copy
        },
    };

    let value = Outer { a: Inner { b: 1 } };
    let got = (b_lens.get)(&value);
    let updated = (b_lens.set)(&value, 2);

    println!("{} {}", got, (b_lens.get)(&updated));
}
