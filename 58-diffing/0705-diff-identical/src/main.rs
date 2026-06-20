use similar::{ChangeTag, TextDiff};

fn main() {
    // Diff A against itself (A -> A): identical inputs have no differences.
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nbanana\ncherry\n";

    let diff = TextDiff::from_lines(a, b);
    let (mut added, mut removed) = (0usize, 0usize);
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Insert => added += 1,
            ChangeTag::Delete => removed += 1,
            ChangeTag::Equal => {}
        }
    }

    println!("{} {}", added, removed); // 0 0
}
