use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    let mut out = Vec::new();
    while let Some(n) = queue.pop_front() {
        out.push(n.to_string());
    }
    println!("{}", out.join(" "));
}
