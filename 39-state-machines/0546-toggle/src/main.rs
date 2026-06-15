use rust_fsm::*;

state_machine! {
    derive(Debug)
    Toggle(Off)
    Off(Toggle) => On,
    On(Toggle) => Off,
}

fn main() {
    let mut m: StateMachine<Toggle> = StateMachine::new();
    // Fire the toggle event 3 times: off -> on -> off -> on.
    for _ in 0..3 {
        m.consume(&ToggleInput::Toggle).unwrap();
    }
    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
