use rust_fsm::*;

// A finite state machine (FSM) defined with the `rust-fsm` crate's
// `state_machine!` macro. Beyond states and transitions, each edge also
// declares an *output* (the `[Tick]` part): rust-fsm emits this value every
// time the transition fires. That output is the per-transition action — we
// count one `Tick` for each successful `consume`, so the running total comes
// from the machine, not from a hardcoded number.
//
// A three-step workflow drives the count: Pending --start--> Running,
// Running --work--> Working, Working --finish--> Done. Each step emits Tick.
state_machine! {
    derive(Debug)
    Workflow(Pending)

    Pending(Start) => Running [Tick],
    Running(Work) => Working [Tick],
    Working(Finish) => Done [Tick],
}

fn main() {
    let mut m: StateMachine<Workflow> = StateMachine::new();

    // Fire a fixed sequence of three valid events.
    let events = [
        WorkflowInput::Start,
        WorkflowInput::Work,
        WorkflowInput::Finish,
    ];

    // The counter is incremented by the per-transition action: every
    // successful consume emits a `Tick` output, and each emitted Tick bumps
    // the count. Nothing here is hardcoded to 3.
    let mut count = 0;
    for event in &events {
        if let Ok(Some(WorkflowOutput::Tick)) = m.consume(event) {
            count += 1;
        }
    }

    println!("{}", count);
}
