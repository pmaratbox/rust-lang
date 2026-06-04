use std::collections::HashMap;

struct LruCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: Vec<i32>,
}

impl LruCache {
    fn new(capacity: usize) -> Self {
        LruCache { capacity, map: HashMap::new(), order: Vec::new() }
    }

    fn touch(&mut self, key: i32) {
        self.order.retain(|&k| k != key);
        self.order.push(key);
    }

    fn put(&mut self, key: i32, value: i32) {
        if !self.map.contains_key(&key) && self.map.len() == self.capacity {
            let lru = self.order.remove(0);
            self.map.remove(&lru);
        }
        self.map.insert(key, value);
        self.touch(key);
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.map.get(&key) {
            self.touch(key);
            value
        } else {
            -1
        }
    }
}

fn main() {
    let mut cache = LruCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.get(1);
    cache.put(3, 3);
    println!("{} {}", cache.get(1), cache.get(2));
}
