use std::collections::HashMap;
use std::hash::Hash;

/// Create and manipulate state machines
#[derive(Debug)]
pub struct Machine<A, S, C> {
    /// Global state of the machine. Can be manipulated with transitions.
    pub context: C,

    /// Current state of the machine.
    pub value: S,

    /// Unique identifier for the machine. Can be reference by other machines.
    pub id: String,

    /// Initial state of the machine. (readonly)
    initial: S,

    /// Available states and transitions for the machine.
    pub states: HashMap<S, Transition<A, S, C>>,
}

impl<A: Copy, S: Eq + Hash + Copy, C: Copy> Machine<A, S, C> {
    /// Create a new state machine
    pub fn new(
        id: String,
        initial: S,
        context: C,
    ) -> Self {
        Machine::<A, S, C> {
            context,
            value: initial,
            id,
            initial,
            states: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, state_name: S, state: Transition<A, S, C>) {
        self.states.insert(state_name, state);
    }

    /// Send an action to the state machine
    pub fn transition(&mut self, action: &A) {
        if let Some(transition) = self.states.get(&self.value) {
            match transition.context {
                Some(fn_context) => {
                    self.context = fn_context(self.context, action.to_owned(), self.value);
                }
                None => {}
            }

            match transition.on {
                Some(fn_on) => {
                    self.value = fn_on(self.context, action.to_owned(), self.value);
                }
                None => {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Transition<A, S, C> {
    /// The state to transition to
    pub on: Option<fn(context: C, action: A, state: S) -> S>,

    /// The action to execute when running this transition
    pub context: Option<fn(context: C, action: A, state: S) -> C>,
}
