#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

const VALUES: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];

fn main() {
    let ordinal_s = Direction::S as usize;
    let name_at_3 = VALUES[3];
    println!("{} {:?}", ordinal_s, name_at_3);
}
