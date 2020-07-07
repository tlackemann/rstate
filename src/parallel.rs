use std::hash::Hash;

use crate::machine::*;

#[derive(Debug)]
pub struct ParallelMachine<A, S, C> {
    pub id: String,
    pub machines: Vec<Machine<A, S, C>>,
    pub value: Vec<S>
}

impl<A: Copy, S: Eq + Hash + Copy, C: Copy> ParallelMachine<A, S, C> {
    /// Create a new state machine
    pub fn new(id: String, machines: Vec<Machine<A, S, C>>) -> Self {
        let value = machines.iter().map(|machine| machine.value).collect();
        ParallelMachine { id, machines, value }
    }

    /// Send an action to the state machines
    pub fn transition(&mut self, action: &A) {
        for machine in self.machines.iter_mut() {
            machine.transition(action);
        }
        self.value = self.machines.iter().map(|machine| machine.value).collect();
    }
}
