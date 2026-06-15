use rust_fsm::*;

state_machine! {
    derive(Debug)
    Workflow(Idle)
    Idle(Submit) => Pending,
    Pending(Approve) => Approved,
}

fn main() {
    let mut m: StateMachine<Workflow> = StateMachine::new();
    m.consume(&WorkflowInput::Submit).unwrap();
    m.consume(&WorkflowInput::Approve).unwrap();
    println!("{}", format!("{:?}", m.state()).to_lowercase());
}
