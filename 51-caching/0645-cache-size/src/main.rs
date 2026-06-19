use lru::LruCache;
use std::num::NonZeroUsize;

fn cap(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn main() {
    let mut cache: LruCache<&str, i32> = LruCache::new(cap(5));
    cache.put("a", 1);
    cache.put("b", 2);
    println!("{}", cache.len());
}
