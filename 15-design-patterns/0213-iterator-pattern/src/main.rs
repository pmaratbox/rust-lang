struct Counter {
    current: i32,
    end: i32,
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.current <= self.end {
            let v = self.current;
            self.current += 1;
            Some(v)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter { current: 1, end: 3 };
    let parts: Vec<String> = counter.map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
