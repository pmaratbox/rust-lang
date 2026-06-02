fn quicksort(items: &[i32]) -> Vec<i32> {
    if items.len() <= 1 {
        return items.to_vec();
    }
    let pivot = items[0];
    let less: Vec<i32> = items[1..].iter().filter(|&&x| x < pivot).copied().collect();
    let greater: Vec<i32> = items[1..].iter().filter(|&&x| x >= pivot).copied().collect();
    let mut result = quicksort(&less);
    result.push(pivot);
    result.extend(quicksort(&greater));
    result
}

fn main() {
    let data = [3, 1, 4, 1, 5, 2];
    let sorted = quicksort(&data);
    let parts: Vec<String> = sorted.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
