use rust_fsm::*;

// Traffic light cycle: red -> green -> yellow -> red, driven by a `Next` event.
state_machine! {
    derive(Debug)
    TrafficLight(Red)
    Red(Next) => Green,
    Green(Next) => Yellow,
    Yellow(Next) => Red,
}

fn main() {
    let mut m: StateMachine<TrafficLight> = StateMachine::new();

    // Fire the fixed event sequence: `Next` twice (red -> green -> yellow).
    m.consume(&TrafficLightInput::Next).unwrap();
    m.consume(&TrafficLightInput::Next).unwrap();

    // Print the resulting state name, lowercased.
    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
