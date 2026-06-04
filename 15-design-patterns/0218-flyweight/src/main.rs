use std::collections::HashMap;
use std::rc::Rc;

struct Glyph {
    ch: char,
}

struct Factory {
    cache: HashMap<char, Rc<Glyph>>,
}
impl Factory {
    fn new() -> Self {
        Factory { cache: HashMap::new() }
    }
    fn get(&mut self, ch: char) -> Rc<Glyph> {
        self.cache
            .entry(ch)
            .or_insert_with(|| Rc::new(Glyph { ch }))
            .clone()
    }
}

fn main() {
    let mut factory = Factory::new();
    for ch in ['a', 'b', 'a'] {
        let _ = factory.get(ch);
    }
    println!("{}", factory.cache.len());
}
