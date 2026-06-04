struct CircularQueue {
    buf: Vec<Option<i32>>,
    head: usize,
    tail: usize,
    len: usize,
    cap: usize,
}

impl CircularQueue {
    fn new(cap: usize) -> Self {
        CircularQueue { buf: vec![None; cap], head: 0, tail: 0, len: 0, cap }
    }

    fn enqueue(&mut self, value: i32) -> bool {
        if self.len == self.cap {
            return false;
        }
        self.buf[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.cap;
        self.len += 1;
        true
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.len == 0 {
            return None;
        }
        let value = self.buf[self.head].take();
        self.head = (self.head + 1) % self.cap;
        self.len -= 1;
        value
    }

    fn contents(&self) -> Vec<i32> {
        (0..self.len)
            .map(|i| self.buf[(self.head + i) % self.cap].unwrap())
            .collect()
    }
}

fn main() {
    let mut q = CircularQueue::new(3);
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.dequeue();
    q.enqueue(4);
    let parts: Vec<String> = q.contents().iter().map(|n| n.to_string()).collect();
    println!("{}", parts.join(" "));
}
