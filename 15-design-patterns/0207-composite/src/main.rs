trait Component {
    fn size(&self) -> i32;
}

struct Leaf {
    value: i32,
}
impl Component for Leaf {
    fn size(&self) -> i32 {
        self.value
    }
}

struct Composite {
    children: Vec<Box<dyn Component>>,
}
impl Component for Composite {
    fn size(&self) -> i32 {
        self.children.iter().map(|c| c.size()).sum()
    }
}

fn main() {
    let root = Composite {
        children: vec![
            Box::new(Leaf { value: 1 }),
            Box::new(Leaf { value: 2 }),
            Box::new(Leaf { value: 3 }),
        ],
    };
    println!("{}", root.size());
}
