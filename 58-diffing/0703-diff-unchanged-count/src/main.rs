// Count unchanged (equal) lines using the `similar` crate's line diff.
use similar::{ChangeTag, TextDiff};

fn main() {
    let a = "apple\nbanana\ncherry\n";
    let b = "apple\nblueberry\ncherry\ndate\n";

    let diff = TextDiff::from_lines(a, b);

    let mut unchanged = 0;
    for change in diff.iter_all_changes() {
        if change.tag() == ChangeTag::Equal {
            unchanged += 1;
        }
    }

    // Number of UNCHANGED lines shared by A and B (apple, cherry).
    println!("{}", unchanged);
}
