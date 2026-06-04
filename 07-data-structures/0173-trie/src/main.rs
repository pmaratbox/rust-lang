use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.end
    }
}

fn main() {
    let mut trie = Trie::default();
    trie.insert("cat");
    trie.insert("car");
    let yes = if trie.search("car") { "yes" } else { "no" };
    let no = if trie.search("can") { "yes" } else { "no" };
    println!("{} {}", yes, no);
}
