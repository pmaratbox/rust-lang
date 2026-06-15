use rust_fsm::*;

// A door modeled as a finite state machine. The `Open` event is only defined
// as a valid transition out of the `Unlocked` state, so the transition is
// "guarded" by the FSM table itself: firing `Open` from any other state is
// rejected.
state_machine! {
    derive(Debug)
    Door(Locked)
    Locked(Unlock) => Unlocked,
    Unlocked(Open) => Open,
}

fn main() {
    let mut m: StateMachine<Door> = StateMachine::new();

    // Fixed event sequence: unlock, then open.
    m.consume(&DoorInput::Unlock).unwrap();
    m.consume(&DoorInput::Open).unwrap();

    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
