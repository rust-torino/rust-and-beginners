pub struct Start;
pub struct State1;
pub struct State2;
pub struct State3;
pub struct End;

impl Default for Start {
    fn default() -> Self {
        Start
    }
}

trait MachineState {}

impl MachineState for Start {}
impl MachineState for State1 {}
impl MachineState for State2 {}
impl MachineState for State3 {}
impl MachineState for End {}

trait HasNextState {
    type Next: MachineState;
    fn next_state(self) -> Self::Next;
}

impl HasNextState for Start {
    type Next = State1;
    fn next_state(self) -> Self::Next {
        State1
    }
}

impl HasNextState for State1 {
    type Next = State2;
    fn next_state(self) -> Self::Next {
        State2
    }
}

impl HasNextState for State2 {
    type Next = State3;
    fn next_state(self) -> Self::Next {
        State3
    }
}

impl HasNextState for State3 {
    type Next = End;
    fn next_state(self) -> Self::Next {
        End
    }
}
