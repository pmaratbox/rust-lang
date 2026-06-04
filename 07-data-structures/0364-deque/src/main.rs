use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    let parts: Vec<String> = deque.iter().map(|n| n.to_string()).collect();
    println!("{}", parts.join(" "));
}
