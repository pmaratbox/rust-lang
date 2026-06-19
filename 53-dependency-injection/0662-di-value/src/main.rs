use shaku::{module, Component, Interface, HasComponent};

// The service interface. The implementation carries a plain configuration
// value that was injected at build time rather than another component.
trait Config: Interface {
    fn value(&self) -> String;
}

// A component with a non-injected field. shaku treats such fields as
// parameters: their value is supplied when the module is built.
#[derive(Component)]
#[shaku(interface = Config)]
struct ConfigImpl {
    value: String,
}

impl Config for ConfigImpl {
    fn value(&self) -> String {
        self.value.clone()
    }
}

// Register the component in the module (the DI container's wiring).
module! {
    AppModule {
        components = [ConfigImpl],
        providers = [],
    }
}

fn main() {
    // Build the container, supplying the constant value `v1` as the
    // ConfigImpl component's parameter.
    let module = AppModule::builder()
        .with_component_parameters::<ConfigImpl>(ConfigImplParameters {
            value: "v1".into(),
        })
        .build();

    // Resolve the service and print the injected value.
    let config: &dyn Config = module.resolve_ref();
    println!("{}", config.value());
}
