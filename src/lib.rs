//! rState
//!
//! A state machine library for Rust, inspired by xstate

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Machine<A, S, C> {
    id: String,
    initial: S,
    current: S,
    context: C,
    states: HashMap<S, HashMap<A, Transition<S, C>>>,
}

impl<A: Eq + Hash, S: Eq + Hash + Copy, C: Copy> Machine<A, S, C> {
    pub fn new(
        id: String,
        initial: S,
        states: HashMap<S, HashMap<A, Transition<S, C>>>,
        context: C,
    ) -> Self {
        Machine::<A, S, C> {
            id,
            initial,
            current: initial,
            context,
            states,
        }
    }

    pub fn send(&mut self, action: A) {
        if let Some(transitions) = self.states.get(&self.current) {
            match transitions.get(&action) {
                Some(transition) => {
                    self.current = transition.state.to_owned();

                    match transition.action {
                        Some(action) => {
                            self.context = action(self.context);
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Transition<S, C> {
    state: S,
    action: Option<fn(context: C) -> C>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn toggle_machine() {
        #[derive(Debug, PartialEq, Eq, Hash)]
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

        let mut active_states: HashMap<Action, Transition<State, Context>> = HashMap::new();
        active_states.insert(
            Action::Toggle,
            Transition {
                action: None,
                state: State::Inactive,
            },
        );

        let mut inactive_states: HashMap<Action, Transition<State, Context>> = HashMap::new();
        inactive_states.insert(
            Action::Toggle,
            Transition {
                action: Some(|mut context| {
                    context.count += 1;
                    context
                }),
                state: State::Active,
            },
        );

        let mut states: HashMap<State, HashMap<Action, Transition<State, Context>>> =
            HashMap::new();
        states.insert(State::Active, active_states);
        states.insert(State::Inactive, inactive_states);

        let context = Context { count: 0 };
        let mut machine = Machine::<Action, State, Context>::new(
            "toggle".to_string(),
            State::Inactive,
            states,
            context,
        );

        assert_eq!(machine.current, State::Inactive);
        assert_eq!(machine.context.count, 0);

        machine.send(Action::Toggle);
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 1);

        machine.send(Action::Toggle);
        assert_eq!(machine.current, State::Inactive);
        assert_eq!(machine.context.count, 1);

        machine.send(Action::Toggle);
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 2);
    }

    fn increment_machine() {
        #[derive(Debug, PartialEq, Eq, Hash)]
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
        let mut active_states: HashMap<Action, Transition<State, Context>> = HashMap::new();
        active_states.insert(
            Action::Increment(1),
            Transition {
                action: Some(|mut context| {
                    context.count += 1;
                    context
                }),
                state: State::Active,
            },
        );

        let mut inactive_states: HashMap<Action, Transition<State, Context>> = HashMap::new();
        inactive_states.insert(
            Action::Decrement(1),
            Transition {
                action: Some(|mut context| {
                    context.count -= 1;
                    context
                }),
                state: State::Active,
            },
        );

        let mut states: HashMap<State, HashMap<Action, Transition<State, Context>>> =
            HashMap::new();
        states.insert(State::Active, active_states);

        // Create a context
        let context = Context { count: 0 };
        let mut machine = Machine::<Action, State, Context>::new(
            "toggle".to_string(),
            State::Active,
            states,
            context,
        );

        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 0);

        machine.send(Action::Increment(1));
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 1);

        machine.send(Action::Decrement(1));
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 0);

        machine.send(Action::Increment(5));
        assert_eq!(machine.current, State::Active);
        assert_eq!(machine.context.count, 5);
    }
}
