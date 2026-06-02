struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let head = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: Some(Box::new(Node { value: 3, next: None })),
        })),
    };

    let mut parts = Vec::new();
    let mut node = Some(&head);
    while let Some(n) = node {
        parts.push(n.value.to_string());
        node = n.next.as_deref();
    }
    println!("{}", parts.join(" -> "));
}
