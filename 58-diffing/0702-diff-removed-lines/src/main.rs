use similar::{ChangeTag, TextDiff};

fn main() {
    // A -> B. The removed lines (present in A, not in B) in document (A) order.
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nblueberry\ncherry\ndate\n";

    let diff = TextDiff::from_lines(a, b);
    let mut removed: Vec<String> = Vec::new();
    for change in diff.iter_all_changes() {
        let line = change.value().trim_end_matches('\n').to_string();
        if let ChangeTag::Delete = change.tag() {
            removed.push(line);
        }
    }

    println!("{}", removed.join(","));
}
