use shaku::{module, Component, HasComponent, Interface};

// The Animal abstraction (interface). Any implementation can be bound to it.
trait Animal: Interface {
    fn sound(&self) -> String;
}

// A concrete Dog implementation of the Animal interface.
#[derive(Component)]
#[shaku(interface = Animal)]
struct Dog;

impl Animal for Dog {
    fn sound(&self) -> String {
        "woof".into()
    }
}

// The container binds the Animal interface to the Dog component.
module! {
    AppModule {
        components = [Dog],
        providers = [],
    }
}

fn main() {
    let module = AppModule::builder().build();

    // Resolve by the interface — the container hands back the bound Dog impl.
    let animal: &dyn Animal = module.resolve_ref();
    println!("{}", animal.sound());
}
