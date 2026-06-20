use uuid::Uuid;

fn main() {
    // The nil UUID: all 128 bits set to zero, rendered in canonical form.
    println!("{}", Uuid::nil());
}
