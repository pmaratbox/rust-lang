// Count removed lines using the `similar` crate's line diff.
use similar::{ChangeTag, TextDiff};

fn main() {
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nblueberry\ncherry\ndate\n";

    let diff = TextDiff::from_lines(a, b);

    let mut removed = 0;
    for change in diff.iter_all_changes() {
        if change.tag() == ChangeTag::Delete {
            removed += 1;
        }
    }

    // Number of REMOVED lines going from A to B.
    println!("{}", removed);
}
