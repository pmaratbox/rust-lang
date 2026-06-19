use lru::LruCache;
use std::num::NonZeroUsize;

fn cap(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn show(opt: Option<&i32>) -> String {
    opt.map(|v| v.to_string()).unwrap_or_else(|| "miss".into())
}

fn main() {
    let mut cache: LruCache<&str, i32> = LruCache::new(cap(3));
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);

    // GET a promotes it to most-recently-used, so `b` becomes the LRU.
    cache.get(&"a");

    // Inserting d evicts the LRU key `b`.
    cache.put("d", 4);

    println!("{} {}", show(cache.get(&"a")), show(cache.get(&"b")));
}
