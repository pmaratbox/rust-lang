struct Resource;

impl Resource {
    fn new() -> Resource {
        println!("open");
        Resource
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("close");
    }
}

fn main() {
    let _resource = Resource::new();
    println!("use");
}
