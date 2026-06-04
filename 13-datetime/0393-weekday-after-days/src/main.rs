fn main() {
    let names = [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
    ];
    let start = 6; // Saturday
    let result = (start + 3) % 7;
    println!("{}", names[result]);
}
