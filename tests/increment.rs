#[cfg(test)]
mod tests {
    use rstate::*;
    use std::hash::Hash;

    #[test]
    fn increment_machine() {
        #[derive(Debug, Copy, Clone)]
        enum Action {
            Increment(u8),
            Decrement(u8),
            Finished,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum State {
            Active,
            Done,
        }

        #[derive(Debug, Clone, Copy)]
        struct Context {
            count: u8,
        }

        // Create a context
        let context = Context { count: 0 };
        let mut machine = Machine::<Action, State, Context>::new(
            "increment".to_string(),
            State::Active,
            context,
        );

        // Define transitions between states from actions
        machine.add_state(
            State::Active,
            Transition {
                on: Some(|_context, action, _state| match action {
                    Action::Finished => State::Done,
                    _ => State::Active,
                }),
                context: Some(|mut context, action, _state| {
                    match action {
                        Action::Increment(val) => context.count += val,
                        Action::Decrement(val) => context.count -= val,
                        _ => {}
                    }
                    context
                }),
                ..Default::default()
            },
        );
        // Define transitions between states from actions
        machine.add_state(
            State::Done,
            Transition {
                on: None,
                context: None,
                ..Default::default()
            },
        );

        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.count, 0);

        machine.transition(&Action::Increment(1));
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.count, 1);

        machine.transition(&Action::Decrement(1));
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.count, 0);

        machine.transition(&Action::Increment(5));
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.count, 5);

        machine.transition(&Action::Finished);
        assert_eq!(machine.value, State::Done);
    }
}
