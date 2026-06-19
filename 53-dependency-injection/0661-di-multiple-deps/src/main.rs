use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait A: Interface {
    fn x(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = A)]
struct AImpl;

impl A for AImpl {
    fn x(&self) -> String {
        "a".into()
    }
}

trait B: Interface {
    fn y(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = B)]
struct BImpl;

impl B for BImpl {
    fn y(&self) -> String {
        "b".into()
    }
}

trait Service: Interface {
    fn run(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(inject)]
    a: Arc<dyn A>,
    #[shaku(inject)]
    b: Arc<dyn B>,
}

impl Service for ServiceImpl {
    fn run(&self) -> String {
        format!("{}{}", self.a.x(), self.b.y())
    }
}

module! {
    AppModule {
        components = [AImpl, BImpl, ServiceImpl],
        providers = []
    }
}

fn main() {
    let m = AppModule::builder().build();
    let svc: &dyn Service = m.resolve_ref();
    println!("{}", svc.run());
}
