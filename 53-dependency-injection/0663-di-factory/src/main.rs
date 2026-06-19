use shaku::{module, Provider, HasProvider};

// The service interface that the factory produces. Unlike a singleton
// `Component`, a *provider* builds a fresh value on every request.
trait Widget {
    fn value(&self) -> String;
}

// The object the factory constructs. `#[derive(Provider)]` turns this into a
// FACTORY for the `Widget` interface: shaku generates a `provide` function
// that constructs (builds) a new instance each time it is asked for, instead
// of storing one shared component.
#[derive(Provider)]
#[shaku(interface = Widget)]
struct BuiltWidget;

impl Widget for BuiltWidget {
    fn value(&self) -> String {
        "built".into()
    }
}

// Register the factory under `providers` (factories), not `components`.
module! {
    AppModule {
        components = [],
        providers = [BuiltWidget],
    }
}

fn main() {
    // Build the container, then ask it to PROVIDE a Widget. The factory runs
    // and constructs the object; we call value() on it and print the result.
    let module = AppModule::builder().build();
    let widget: Box<dyn Widget> = module.provide().unwrap();
    println!("{}", widget.value());
}
