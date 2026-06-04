struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let min = self.data.pop();
        let len = self.data.len();
        let mut i = 0;
        loop {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut smallest = i;
            if l < len && self.data[l] < self.data[smallest] {
                smallest = l;
            }
            if r < len && self.data[r] < self.data[smallest] {
                smallest = r;
            }
            if smallest == i {
                break;
            }
            self.data.swap(i, smallest);
            i = smallest;
        }
        min
    }
}

fn main() {
    let mut heap = MinHeap::new();
    for v in [3, 1, 2] {
        heap.push(v);
    }
    let mut out = Vec::new();
    while let Some(v) = heap.pop() {
        out.push(v.to_string());
    }
    println!("{}", out.join(" "));
}
