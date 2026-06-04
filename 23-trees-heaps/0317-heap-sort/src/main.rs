fn sift_down(a: &mut [i64], mut i: usize, n: usize) {
    loop {
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut largest = i;
        if l < n && a[l] > a[largest] {
            largest = l;
        }
        if r < n && a[r] > a[largest] {
            largest = r;
        }
        if largest == i {
            break;
        }
        a.swap(i, largest);
        i = largest;
    }
}

fn heap_sort(a: &mut Vec<i64>) {
    let n = a.len();
    if n < 2 {
        return;
    }
    // Build max-heap.
    for i in (0..n / 2).rev() {
        sift_down(a, i, n);
    }
    // Repeatedly move max to the end.
    for end in (1..n).rev() {
        a.swap(0, end);
        sift_down(a, 0, end);
    }
}

fn main() {
    let mut data = vec![5, 3, 8, 1, 4];
    heap_sort(&mut data);
    let parts: Vec<String> = data.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
