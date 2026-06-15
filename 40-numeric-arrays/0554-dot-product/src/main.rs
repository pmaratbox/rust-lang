use ndarray::{array, Array1};

fn main() {
    let a: Array1<i32> = array![1, 2, 3];
    let b: Array1<i32> = array![4, 5, 6];
    let d = a.dot(&b);
    println!("{}", d);
}
