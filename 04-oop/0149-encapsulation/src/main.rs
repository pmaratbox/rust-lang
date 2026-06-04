struct Account {
    balance: u64,
}

impl Account {
    fn new(initial: u64) -> Self {
        Account { balance: initial }
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn balance(&self) -> u64 {
        self.balance
    }
}

fn main() {
    let mut account = Account::new(100);
    account.deposit(50);
    println!("{}", account.balance());
}
