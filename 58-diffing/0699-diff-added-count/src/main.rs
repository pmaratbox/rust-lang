// Count added lines using the `similar` crate's line diff.
use similar::{ChangeTag, TextDiff};

fn main() {
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nblueberry\ncherry\ndate\n";

    let diff = TextDiff::from_lines(a, b);

    let mut added = 0;
    for change in diff.iter_all_changes() {
        if change.tag() == ChangeTag::Insert {
            added += 1;
        }
    }

    // Number of ADDED lines going from A to B.
    println!("{}", added);
}
