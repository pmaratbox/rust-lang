use lru::LruCache;
use std::num::NonZeroUsize;

fn cap(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn main() {
    let mut cache: LruCache<&str, i32> = LruCache::new(cap(3));
    cache.put("a", 1);
    // contains(&k) checks membership WITHOUT promoting recency
    println!("{} {}", cache.contains(&"a"), cache.contains(&"x"));
}
