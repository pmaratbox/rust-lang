trait Node {
    fn accept(&self, visitor: &mut dyn Visitor);
}

struct Leaf {
    value: i32,
}
impl Node for Leaf {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_leaf(self);
    }
}

struct Tree {
    children: Vec<Box<dyn Node>>,
}
impl Node for Tree {
    fn accept(&self, visitor: &mut dyn Visitor) {
        for child in &self.children {
            child.accept(visitor);
        }
    }
}

trait Visitor {
    fn visit_leaf(&mut self, leaf: &Leaf);
}

struct SumVisitor {
    total: i32,
}
impl Visitor for SumVisitor {
    fn visit_leaf(&mut self, leaf: &Leaf) {
        self.total += leaf.value;
    }
}

fn main() {
    let tree = Tree {
        children: vec![
            Box::new(Leaf { value: 1 }),
            Box::new(Leaf { value: 2 }),
            Box::new(Leaf { value: 3 }),
        ],
    };
    let mut visitor = SumVisitor { total: 0 };
    tree.accept(&mut visitor);
    println!("{}", visitor.total);
}
