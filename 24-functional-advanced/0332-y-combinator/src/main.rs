// A fixed-point combinator: given a non-recursive generator `f` that receives
// "itself" as its first argument, `fix` ties the knot without named recursion.
struct Rec<'a, A, B>(&'a dyn Fn(&Rec<A, B>, A) -> B);

fn fix<A, B>(f: &dyn Fn(&dyn Fn(A) -> B, A) -> B, x: A) -> B {
    let rec = Rec(&|r, a| f(&|n| (r.0)(r, n), a));
    (rec.0)(&rec, x)
}

fn main() {
    // factorial generator: takes `rec` (itself) and n, no self-reference by name.
    let fac_gen = |rec: &dyn Fn(u64) -> u64, n: u64| if n == 0 { 1 } else { n * rec(n - 1) };
    let result = fix(&fac_gen, 5);
    println!("{}", result);
}
