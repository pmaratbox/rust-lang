trait Button {
    fn label(&self) -> String;
}
trait Checkbox {
    fn label(&self) -> String;
}

struct DarkButton;
struct DarkCheckbox;

impl Button for DarkButton {
    fn label(&self) -> String {
        "dark-button".to_string()
    }
}
impl Checkbox for DarkCheckbox {
    fn label(&self) -> String {
        "dark-checkbox".to_string()
    }
}

trait ThemeFactory {
    fn button(&self) -> Box<dyn Button>;
    fn checkbox(&self) -> Box<dyn Checkbox>;
}

struct DarkFactory;
impl ThemeFactory for DarkFactory {
    fn button(&self) -> Box<dyn Button> {
        Box::new(DarkButton)
    }
    fn checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(DarkCheckbox)
    }
}

fn main() {
    let f: Box<dyn ThemeFactory> = Box::new(DarkFactory);
    println!("{} {}", f.button().label(), f.checkbox().label());
}
