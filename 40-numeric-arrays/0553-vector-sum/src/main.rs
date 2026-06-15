use ndarray::{array, Array1};

fn main() {
    let v: Array1<i32> = array![1, 2, 3, 4];
    let total: i32 = v.sum();
    println!("{}", total);
}
