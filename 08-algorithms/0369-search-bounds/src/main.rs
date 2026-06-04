fn lower_bound(a: &[i32], target: i32) -> usize {
    a.partition_point(|&x| x < target)
}

fn upper_bound(a: &[i32], target: i32) -> usize {
    a.partition_point(|&x| x <= target)
}

fn main() {
    let a = [1, 3, 5, 5, 7];
    println!("{} {}", lower_bound(&a, 5), upper_bound(&a, 5));
}
