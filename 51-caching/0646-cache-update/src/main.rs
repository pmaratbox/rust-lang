use lru::LruCache;
use std::num::NonZeroUsize;

fn cap(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn main() {
    let mut cache: LruCache<&str, i32> = LruCache::new(cap(3));
    cache.put("a", 1);
    cache.put("a", 2); // same key — updates the stored value in place
    let out = cache
        .get(&"a")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "miss".into());
    println!("{}", out);
}
