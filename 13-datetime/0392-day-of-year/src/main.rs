fn main() {
    let days = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let (month, day) = (3, 1);
    let doy: u32 = days[..(month - 1)].iter().sum::<u32>() + day as u32;
    println!("{}", doy);
}
