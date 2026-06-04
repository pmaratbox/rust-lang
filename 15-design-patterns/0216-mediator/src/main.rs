struct ColleagueB;
impl ColleagueB {
    fn receive(&self, msg: &str) {
        println!("B got: {}", msg);
    }
}

struct Mediator {
    b: ColleagueB,
}
impl Mediator {
    fn send(&self, msg: &str) {
        self.b.receive(msg);
    }
}

struct ColleagueA<'a> {
    mediator: &'a Mediator,
}
impl<'a> ColleagueA<'a> {
    fn send(&self, msg: &str) {
        self.mediator.send(msg);
    }
}

fn main() {
    let mediator = Mediator { b: ColleagueB };
    let a = ColleagueA { mediator: &mediator };
    a.send("hi");
}
