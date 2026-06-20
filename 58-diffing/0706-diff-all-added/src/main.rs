// Diff an EMPTY list against [x, y] using the `similar` crate's line diff.
use similar::{ChangeTag, TextDiff};

fn main() {
    let a = "";
    let b = "x\ny\n";

    let diff = TextDiff::from_lines(a, b);

    let mut added = 0;
    for change in diff.iter_all_changes() {
        if change.tag() == ChangeTag::Insert {
            added += 1;
        }
    }

    // Diffing from empty: every line in B is added.
    println!("{}", added);
}
