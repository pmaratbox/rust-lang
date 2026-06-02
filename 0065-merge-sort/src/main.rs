fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

fn merge_sort(items: &[i32]) -> Vec<i32> {
    if items.len() <= 1 {
        return items.to_vec();
    }
    let mid = items.len() / 2;
    merge(merge_sort(&items[..mid]), merge_sort(&items[mid..]))
}

fn main() {
    let data = [3, 1, 4, 1, 5, 2];
    let sorted = merge_sort(&data);
    let parts: Vec<String> = sorted.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
