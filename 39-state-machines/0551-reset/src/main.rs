use rust_fsm::*;

// A finite state machine (FSM) defined declaratively with the `rust-fsm`
// crate's `state_machine!` macro. A process is `Idle` until `Start`, which
// moves it to `Running`; a `Reset` event returns it to the initial `Idle`
// state. This shows a transition that loops back to the start state.
state_machine! {
    derive(Debug)
    Process(Idle)
    Idle(Start) => Running,
    Running(Reset) => Idle,
}

fn main() {
    let mut m: StateMachine<Process> = StateMachine::new();

    // Fire the fixed event sequence: start, then reset.
    m.consume(&ProcessInput::Start).unwrap();
    m.consume(&ProcessInput::Reset).unwrap();

    // The resulting state comes from the machine, lowercased.
    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
