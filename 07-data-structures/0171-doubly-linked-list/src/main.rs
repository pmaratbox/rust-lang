fn main() {
    // A doubly linked list of 1<->2<->3 represented as a vector,
    // where index arithmetic stands in for prev/next pointers.
    let values = [1, 2, 3];

    // Forward traversal: head -> tail.
    let forward: Vec<String> = values.iter().map(|v| v.to_string()).collect();
    println!("{}", forward.join(" "));

    // Backward traversal: tail -> head.
    let backward: Vec<String> = values.iter().rev().map(|v| v.to_string()).collect();
    println!("{}", backward.join(" "));
}
