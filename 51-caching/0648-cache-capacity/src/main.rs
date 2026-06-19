use lru::LruCache;
use std::num::NonZeroUsize;

fn cap(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn main() {
    // Capacity 3: putting a 4th item evicts the LRU, so size stays capped at 3.
    let mut c: LruCache<&str, i32> = LruCache::new(cap(3));
    for (k, v) in [("a", 1), ("b", 2), ("c", 3), ("d", 4)] {
        c.put(k, v);
    }
    println!("{}", c.len());
}
