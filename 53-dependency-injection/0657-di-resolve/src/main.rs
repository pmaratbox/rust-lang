use shaku::{module, Component, Interface, HasComponent};

// The service interface (a trait that shaku can bind a component to).
trait Greeter: Interface {
    fn greet(&self) -> String;
}

// A component implementing the interface. `#[derive(Component)]` plus the
// `interface` attribute tells shaku how to build it and what trait it provides.
#[derive(Component)]
#[shaku(interface = Greeter)]
struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn greet(&self) -> String {
        "hello".into()
    }
}

// Register the component in a module (the DI container's wiring).
module! {
    AppModule {
        components = [GreeterImpl],
        providers = [],
    }
}

fn main() {
    // Build the container and resolve the Greeter service out of it.
    let module = AppModule::builder().build();
    let greeter: &dyn Greeter = module.resolve_ref();
    println!("{}", greeter.greet());
}
