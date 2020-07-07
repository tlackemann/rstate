#[cfg(test)]
mod tests {
    use rstate::*;
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

        let context = Context {};
        let mut machine = Machine::<Action, State, Context>::new(
            "light".to_string(),
            State::Green,
            context,
        );

        machine.add_state(
            State::Green,
            Transition {
                context: None,
                on: Some(|_context, action, _state| match action {
                    Action::Timer => State::Yellow,
                }),
            },
        );
        machine.add_state(
            State::Yellow,
            Transition {
                context: None,
                on: Some(|_context, action, _state| match action {
                    Action::Timer => State::Red,
                }),
            },
        );
        machine.add_state(
            State::Red,
            Transition {
                context: None,
                on: Some(|_context, action, _state| match action {
                    Action::Timer => State::Green,
                }),
            },
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
