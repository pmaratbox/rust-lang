struct UserId(u64);
struct ProductId(u64);

fn main() {
    let user = UserId(1);
    let product = ProductId(2);
    println!("user-{} prod-{}", user.0, product.0);
}
