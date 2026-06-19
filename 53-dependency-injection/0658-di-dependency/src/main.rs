use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait Repo: Interface {
    fn data(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Repo)]
struct RepoImpl;

impl Repo for RepoImpl {
    fn data(&self) -> String {
        "data".into()
    }
}

trait Service: Interface {
    fn run(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Service)]
struct ServiceImpl {
    #[shaku(inject)]
    repo: Arc<dyn Repo>,
}

impl Service for ServiceImpl {
    fn run(&self) -> String {
        self.repo.data()
    }
}

module! {
    AppModule {
        components = [RepoImpl, ServiceImpl],
        providers = [],
    }
}

fn main() {
    let module = AppModule::builder().build();
    let service: &dyn Service = module.resolve_ref();
    println!("{}", service.run());
}
