use std::hash::Hash;
use std::fmt::Debug;

use crate::machine::*;

#[derive(Debug)]
pub struct HistoryMachine<A, S, C> {
    pub machine: Machine<A, S, C>,
    pub past: Vec<(S, C)>,
    pub future: Vec<(S, C)>,
}

impl<A: Copy, S: Eq + Hash + Copy, C: Debug + Copy> HistoryMachine<A, S, C> {
    /// Create a new state machine
    pub fn new(machine: Machine<A, S, C>) -> Self {
        HistoryMachine { machine, future: vec![], past: vec![] }
    }

    /// Send an action to the state machines
    pub fn transition(&mut self, action: &A) {
        self.past.push((
                self.machine.value,
                self.machine.context
        ));
        self.machine.transition(action);
    }

    pub fn undo(&mut self) {
        if self.past.len() > 0 {
            if let Some((state, context)) = self.past.pop() {
                self.future.push((
                        self.machine.value,
                        self.machine.context
                ));

                self.machine.set_state(state);
                self.machine.set_context(context);
            }
        }
    }

    pub fn redo(&mut self) {
        if self.future.len() > 0 {
            if let Some((state, context)) = self.future.pop() {
                self.past.push((
                        self.machine.value,
                        self.machine.context
                ));

                self.machine.set_state(state);
                self.machine.set_context(context);
            }
        }
    }
}
