#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn states_chain() {
        let start = Start::default();
        let state1: State1 = start.next_state();
        let state2: State2 = state1.next_state();
        let state3: State3 = state2.next_state();
        let _: End = state3.next_state();
    }
}
