//! rState
//!
//! A state machine library for Rust, inspired by xstate

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Machine<A, S, C> {
    context: C,
    current: S,
    id: String,
    initial: S,
    transitions: HashMap<S, Transition<A, S, C>>,
}

impl<A: Copy + Eq + Hash, S: Eq + Hash + Copy, C: Copy> Machine<A, S, C> {
    pub fn new(
        id: String,
        initial: S,
        transitions: HashMap<S, Transition<A, S, C>>,
        context: C,
    ) -> Self {
        Machine::<A, S, C> {
            context,
            current: initial,
            id,
            initial,
            transitions,
        }
    }

    pub fn send(&mut self, action: &A) {
        if let Some(transition) = self.transitions.get(&self.current) {
            match transition.state {
                Some(fn_state) => {
                    self.current = fn_state(self.current, action.to_owned());
                }
                None => {}
            }

            match transition.context {
                Some(fn_context) => {
                    self.context = fn_context(self.context, action.to_owned());
                }
                None => {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Transition<A, S, C> {
    /// The state to transition to
    state: Option<fn(state: S, action: A) -> S>,

    /// The action to execute when running this transition
    context: Option<fn(context: C, action: A) -> C>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn toggle_machine() {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        enum Action {
            Toggle,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum State {
            Active,
            Inactive,
        }

        #[derive(Debug, Clone, Copy)]
        struct Context {
            count: u8,
        }

        let mut states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
        states.insert(
            State::Active,
            Transition {
                context: None,
                state: Some(|_state, action| {
                    match action {
                        Action::Toggle => State::Inactive
                    }
                }),
            },
        );
        states.insert(
            State::Inactive,
            Transition {
                context: Some(|mut context, _action| {
                    context.count += 1;
                    context
                }),
                state: Some(|_state, action| {
                    match action {
                        Action::Toggle => State::Active
                    }
                }),
            },
        );

        let context = Context { count: 0 };
        let mut machine = Machine::<Action, State, Context>::new(
            "toggle".to_string(),
            State::Inactive,
            states,
            context,
        );

        assert_eq!(machine.current, State::Inactive);
        assert_eq!(machine.context.count, 0);

        machine.send(&Action::Toggle);
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 1);

        machine.send(&Action::Toggle);
        assert_eq!(machine.current, State::Inactive);
        assert_eq!(machine.context.count, 1);

        machine.send(&Action::Toggle);
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 2);
    }

    #[test]
    fn increment_machine() {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        enum Action {
            Increment(u8),
            Decrement(u8),
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum State {
            Active,
        }

        #[derive(Debug, Clone, Copy)]
        struct Context {
            count: u8,
        }

        // Define transitions between states from actions
        let mut states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
        states.insert(
            State::Active,
            Transition {
                state: Some(|_state, _action| State::Active),
                context: Some(|mut context, action| {
                    match action {
                        Action::Increment(val) => context.count += val,
                        Action::Decrement(val) => context.count -= val,
                    }
                    context
                }),
            },
        );

        // Create a context
        let context = Context { count: 0 };
        let mut machine = Machine::<Action, State, Context>::new(
            "increment".to_string(),
            State::Active,
            states,
            context,
        );

        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 0);

        machine.send(&Action::Increment(1));
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 1);

        machine.send(&Action::Decrement(1));
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 0);

        machine.send(&Action::Increment(5));
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 5);
    }
}
