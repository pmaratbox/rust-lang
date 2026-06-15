use rust_fsm::*;

// A finite state machine (FSM) defined declaratively with the `rust-fsm`
// crate's `state_machine!` macro: a turnstile that is `Locked` until a
// coin is inserted, then `Unlocked` until someone pushes through.
state_machine! {
    derive(Debug)
    Turnstile(Locked)
    Locked(Coin) => Unlocked,
    Unlocked(Push) => Locked,
}

fn main() {
    let mut m: StateMachine<Turnstile> = StateMachine::new();

    // Fire the fixed event sequence: a single coin.
    m.consume(&TurnstileInput::Coin).unwrap();

    // The resulting state comes from the machine, lowercased.
    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
