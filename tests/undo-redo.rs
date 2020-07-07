#[cfg(test)]
mod tests {
    use rstate::*;
    use std::hash::Hash;

    #[test]
    fn history_machine() {
        #[derive(Debug, Copy, Clone)]
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
                on: Some(|_context, _action, _state| State::Active),
                context: Some(|mut context, action, _state| {
                    match action {
                        Action::Increment(val) => context.count += val,
                        Action::Decrement(val) => context.count -= val,
                    }
                    context
                }),
            },
        );

        let mut history_machine = HistoryMachine::<Action, State, Context>::new(
            machine
        );

        assert_eq!(history_machine.machine.context.count, 0);

        history_machine.transition(&Action::Increment(2));
        assert_eq!(history_machine.machine.context.count, 2);

        history_machine.transition(&Action::Decrement(1));
        assert_eq!(history_machine.machine.context.count, 1);

        history_machine.undo();
        assert_eq!(history_machine.machine.context.count, 2);

        history_machine.undo();
        assert_eq!(history_machine.machine.context.count, 0);

        history_machine.redo();
        assert_eq!(history_machine.machine.context.count, 2);

        history_machine.redo();
        assert_eq!(history_machine.machine.context.count, 1);
    }
}
