use lru::LruCache;
use std::num::NonZeroUsize;

fn main() {
    // Strict LRU cache, capacity 3 (lru crate).
    let mut cache: LruCache<&str, i32> = LruCache::new(NonZeroUsize::new(3).unwrap());

    // The cache is empty, so looking up `x` returns None.
    let result = cache
        .get(&"x")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "miss".into());

    println!("{}", result);
}
