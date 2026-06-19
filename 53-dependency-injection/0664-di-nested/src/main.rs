use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait A: Interface {
    fn v(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = A)]
struct AImpl;

impl A for AImpl {
    fn v(&self) -> String {
        "a".into()
    }
}

trait B: Interface {
    fn v(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = B)]
struct BImpl {
    #[shaku(inject)]
    a: Arc<dyn A>,
}

impl B for BImpl {
    fn v(&self) -> String {
        format!("{}b", self.a.v())
    }
}

trait C: Interface {
    fn v(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = C)]
struct CImpl {
    #[shaku(inject)]
    b: Arc<dyn B>,
}

impl C for CImpl {
    fn v(&self) -> String {
        format!("{}c", self.b.v())
    }
}

module! {
    AppModule {
        components = [AImpl, BImpl, CImpl],
        providers = []
    }
}

fn main() {
    let m = AppModule::builder().build();
    let c: &dyn C = m.resolve_ref();
    println!("{}", c.v());
}
