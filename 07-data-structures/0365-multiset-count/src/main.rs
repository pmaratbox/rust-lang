use std::collections::HashMap;

struct Multiset {
    counts: HashMap<i32, u32>,
}

impl Multiset {
    fn new() -> Self {
        Multiset { counts: HashMap::new() }
    }

    fn add(&mut self, value: i32) {
        *self.counts.entry(value).or_insert(0) += 1;
    }

    fn remove(&mut self, value: i32) {
        if let Some(count) = self.counts.get_mut(&value) {
            *count -= 1;
            if *count == 0 {
                self.counts.remove(&value);
            }
        }
    }

    fn count(&self, value: i32) -> u32 {
        *self.counts.get(&value).unwrap_or(&0)
    }
}

fn main() {
    let mut set = Multiset::new();
    set.add(1);
    set.add(1);
    set.add(2);
    let before = set.count(1);
    set.remove(1);
    let after = set.count(1);
    println!("{} {}", before, after);
}
