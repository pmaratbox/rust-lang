struct RingBuffer {
    data: [i32; 3],
    head: usize,
    len: usize,
}

impl RingBuffer {
    fn new() -> Self {
        RingBuffer { data: [0; 3], head: 0, len: 0 }
    }

    fn push(&mut self, value: i32) {
        let cap = self.data.len();
        let tail = (self.head + self.len) % cap;
        self.data[tail] = value;
        if self.len == cap {
            // Overwrite oldest: advance head.
            self.head = (self.head + 1) % cap;
        } else {
            self.len += 1;
        }
    }

    fn contents(&self) -> Vec<i32> {
        let cap = self.data.len();
        (0..self.len).map(|i| self.data[(self.head + i) % cap]).collect()
    }
}

fn main() {
    let mut rb = RingBuffer::new();
    for v in [1, 2, 3, 4, 5] {
        rb.push(v);
    }
    let out: Vec<String> = rb.contents().iter().map(|v| v.to_string()).collect();
    println!("{}", out.join(" "));
}
