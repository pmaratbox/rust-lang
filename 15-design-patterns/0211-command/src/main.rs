trait Command {
    fn execute(&self, counter: &mut i32);
    fn undo(&self, counter: &mut i32);
}

struct AddCommand {
    amount: i32,
}
impl Command for AddCommand {
    fn execute(&self, counter: &mut i32) {
        *counter += self.amount;
    }
    fn undo(&self, counter: &mut i32) {
        *counter -= self.amount;
    }
}

fn main() {
    let mut counter = 0;
    let cmd = AddCommand { amount: 5 };
    cmd.execute(&mut counter);
    print!("{} ", counter);
    cmd.undo(&mut counter);
    println!("{}", counter);
}
