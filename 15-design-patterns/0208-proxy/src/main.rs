struct RealSubject;
impl RealSubject {
    fn request(&self) -> &str {
        "loaded"
    }
}

struct Proxy {
    real: Option<RealSubject>,
}

impl Proxy {
    fn new() -> Self {
        Proxy { real: None }
    }
    fn request(&mut self) -> &str {
        if self.real.is_none() {
            self.real = Some(RealSubject);
        }
        self.real.as_ref().unwrap().request()
    }
}

fn main() {
    let mut proxy = Proxy::new();
    println!("{}", proxy.request());
}
