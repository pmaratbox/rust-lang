use rust_fsm::*;

// A finite state machine (FSM) defined declaratively with the `rust-fsm`
// crate's `state_machine!` macro: the classic turnstile, `Locked` until a
// coin is inserted and `Unlocked` until someone pushes through.
state_machine! {
    derive(Debug)
    Turnstile(Locked)
    Locked(Coin) => Unlocked,
    Unlocked(Push) => Locked,
}

fn main() {
    let mut m: StateMachine<Turnstile> = StateMachine::new();

    // Fire `Push` from `Locked`: there is NO transition for this pair, so
    // the machine rejects the event (returns `Err`). We catch and ignore
    // it instead of crashing — the state is left unchanged.
    let _ = m.consume(&TurnstileInput::Push);

    // The resulting state comes from the machine, lowercased: still `locked`.
    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
