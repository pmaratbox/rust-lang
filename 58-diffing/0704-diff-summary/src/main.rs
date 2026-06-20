// Diff summary using the `similar` crate's line diff.
use similar::{ChangeTag, TextDiff};

fn main() {
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nblueberry\ncherry\ndate\n";

    let diff = TextDiff::from_lines(a, b);

    let mut added = 0;
    let mut removed = 0;
    let mut unchanged = 0;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Insert => added += 1,
            ChangeTag::Delete => removed += 1,
            ChangeTag::Equal => unchanged += 1,
        }
    }

    // <added> <removed> <unchanged> from the real LCS-based diff.
    println!("{} {} {}", added, removed, unchanged);
}
