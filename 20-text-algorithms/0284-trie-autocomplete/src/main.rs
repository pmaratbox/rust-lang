use std::collections::BTreeMap;

#[derive(Default)]
struct Node {
    children: BTreeMap<char, Node>,
    end: bool,
}

impl Node {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.end = true;
    }

    fn collect(&self, prefix: &mut String, out: &mut Vec<String>) {
        if self.end {
            out.push(prefix.clone());
        }
        for (&c, child) in &self.children {
            prefix.push(c);
            child.collect(prefix, out);
            prefix.pop();
        }
    }
}

fn autocomplete(root: &Node, prefix: &str) -> Vec<String> {
    let mut node = root;
    for c in prefix.chars() {
        match node.children.get(&c) {
            Some(n) => node = n,
            None => return Vec::new(),
        }
    }
    let mut out = Vec::new();
    let mut buf = prefix.to_string();
    node.collect(&mut buf, &mut out);
    out
}

fn main() {
    let mut root = Node::default();
    for w in ["car", "card", "dog"] {
        root.insert(w);
    }
    println!("{}", autocomplete(&root, "car").join(" "));
}
