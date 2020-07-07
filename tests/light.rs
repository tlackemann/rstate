#[cfg(test)]
mod tests {
    use rstate::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn toggle_machine() {
        #[derive(Copy, Clone, Debug)]
        enum Action {
            Timer,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum State {
            Green,
            Yellow,
            Red,
        }

        #[derive(Debug, Clone, Copy)]
        struct Context {
        }

        let mut states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
        states.insert(
            State::Green,
            Transition {
                context: None,
                on: Some(|context, action, state| match action {
                    Action::Timer => State::Yellow,
                }),
            },
        );
        states.insert(
            State::Yellow,
            Transition {
                context: None,
                on: Some(|context, action, state| match action {
                    Action::Timer => State::Red,
                }),
            },
        );
        states.insert(
            State::Red,
            Transition {
                context: None,
                on: Some(|context, action, state| match action {
                    Action::Timer => State::Green,
                }),
            },
        );

        let context = Context {};
        let mut machine = Machine::<Action, State, Context>::new(
            "light".to_string(),
            State::Green,
            states,
            context,
        );

        assert_eq!(machine.value, State::Green);
        machine.transition(&Action::Timer);
        assert_eq!(machine.value, State::Yellow);
        machine.transition(&Action::Timer);
        assert_eq!(machine.value, State::Red);
        machine.transition(&Action::Timer);
        assert_eq!(machine.value, State::Green);
    }
}
