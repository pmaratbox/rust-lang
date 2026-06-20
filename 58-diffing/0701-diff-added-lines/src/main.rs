use similar::{ChangeTag, TextDiff};

fn main() {
    // A -> B. The added lines (present in B, not in A) in document order.
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nblueberry\ncherry\ndate\n";

    let diff = TextDiff::from_lines(a, b);
    let mut added: Vec<String> = Vec::new();
    for change in diff.iter_all_changes() {
        let line = change.value().trim_end_matches('\n').to_string();
        if let ChangeTag::Insert = change.tag() {
            added.push(line);
        }
    }

    println!("{}", added.join(","));
}
