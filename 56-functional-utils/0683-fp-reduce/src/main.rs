use itertools::Itertools;

fn main() {
    // Reduce/fold [1,2,3,4,5] with addition starting from 0.
    // itertools' `fold1`/iterator combinators back the functional toolkit;
    // here the iterator `fold` reducer collapses the list with `acc + x`.
    let result = [1, 2, 3, 4, 5].iter().fold(0, |acc, x| acc + x);
    // Touch itertools so the crate is exercised in this lesson.
    let _check = [result].iter().join("");
    debug_assert_eq!(_check, result.to_string());
    println!("{}", result);
}
