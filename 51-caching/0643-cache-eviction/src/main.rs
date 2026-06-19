use lru::LruCache;
use std::num::NonZeroUsize;

fn cap(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn main() {
    // Capacity 3, strict LRU. Insert a,b,c,d with no gets in between.
    // The least-recently-used key `a` is evicted when `d` is inserted.
    let mut cache: LruCache<&str, i32> = LruCache::new(cap(3));
    for (k, v) in [("a", 1), ("b", 2), ("c", 3), ("d", 4)] {
        cache.put(k, v);
    }

    let a = cache.get(&"a").map(|v| v.to_string()).unwrap_or("miss".into());
    let d = cache.get(&"d").map(|v| v.to_string()).unwrap_or("miss".into());
    println!("{} {}", a, d);
}
