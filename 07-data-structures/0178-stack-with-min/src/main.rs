struct MinStack {
    data: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { data: Vec::new(), mins: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        let new_min = match self.mins.last() {
            Some(&m) if m < value => m,
            _ => value,
        };
        self.data.push(value);
        self.mins.push(new_min);
    }

    fn get_min(&self) -> Option<i32> {
        self.mins.last().copied()
    }
}

fn main() {
    let mut stack = MinStack::new();
    for v in [3, 1, 2] {
        stack.push(v);
    }
    println!("min: {}", stack.get_min().unwrap());
}
