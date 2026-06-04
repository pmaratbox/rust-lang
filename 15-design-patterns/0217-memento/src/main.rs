struct Memento {
    state: i32,
}

struct Originator {
    state: i32,
}
impl Originator {
    fn save(&self) -> Memento {
        Memento { state: self.state }
    }
    fn restore(&mut self, m: &Memento) {
        self.state = m.state;
    }
}

fn main() {
    let mut originator = Originator { state: 1 };
    let memento = originator.save();
    originator.state = 2;
    print!("{} ", originator.state);
    originator.restore(&memento);
    println!("{}", originator.state);
}
