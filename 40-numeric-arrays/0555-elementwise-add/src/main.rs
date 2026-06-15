use ndarray::{array, Array1};

fn main() {
    let a: Array1<i32> = array![1, 2, 3];
    let b: Array1<i32> = array![10, 20, 30];
    let c = &a + &b;
    println!(
        "{}",
        c.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
    );
}
