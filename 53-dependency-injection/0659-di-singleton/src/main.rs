// shaku — register Repo as a singleton component and resolve it twice.
// shaku stores one Arc per component in the module, so resolving the same
// interface twice yields the SAME instance. We compare the two trait-object
// pointers for identity and print the lowercase bool.
use shaku::{module, Component, Interface, HasComponent};

trait Repo: Interface {
    fn data(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Repo)]
struct RepoImpl;

impl Repo for RepoImpl {
    fn data(&self) -> String {
        "rows".into()
    }
}

module! {
    AppModule {
        components = [RepoImpl],
        providers = []
    }
}

fn main() {
    let module = AppModule::builder().build();

    // Resolve the singleton twice through the container.
    let a: &dyn Repo = module.resolve_ref();
    let b: &dyn Repo = module.resolve_ref();

    // Same registered instance -> identical underlying pointer.
    let same = std::ptr::eq(
        a as *const dyn Repo as *const (),
        b as *const dyn Repo as *const (),
    );
    println!("{}", same);
}
