fn partition(a: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = a[hi];
    let mut i = lo;
    for j in lo..hi {
        if a[j] < pivot {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, hi);
    i
}

fn quickselect(a: &mut [i32], lo: usize, hi: usize, k: usize) -> i32 {
    if lo == hi {
        return a[lo];
    }
    let p = partition(a, lo, hi);
    if k == p {
        a[p]
    } else if k < p {
        quickselect(a, lo, p - 1, k)
    } else {
        quickselect(a, p + 1, hi, k)
    }
}

fn main() {
    let mut a = [7, 10, 4, 3, 20, 15];
    let n = a.len();
    // 3rd smallest -> index 2
    let result = quickselect(&mut a, 0, n - 1, 2);
    println!("{}", result);
}
