struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn expr(&mut self) -> i64 {
        let mut v = self.term();
        while self.peek() == Some('+') {
            self.pos += 1;
            v += self.term();
        }
        v
    }

    fn term(&mut self) -> i64 {
        let mut v = self.factor();
        while self.peek() == Some('*') {
            self.pos += 1;
            v *= self.factor();
        }
        v
    }

    fn factor(&mut self) -> i64 {
        let mut n = 0i64;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                n = n * 10 + (c as i64 - '0' as i64);
                self.pos += 1;
            } else {
                break;
            }
        }
        n
    }
}

fn main() {
    let mut p = Parser {
        chars: "2+3*4".chars().collect(),
        pos: 0,
    };
    println!("{}", p.expr());
}
