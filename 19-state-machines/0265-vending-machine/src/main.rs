fn main() {
    let price = 25;
    let mut total = 0;
    for coin in [10, 10, 5] {
        total += coin;
        if total >= price {
            println!("dispensed");
            break;
        }
    }
}
