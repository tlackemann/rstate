#[cfg(test)]
mod tests {
    use rstate::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn toggle_machine() {
        #[derive(Copy, Clone, Debug)]
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
                on: Some(|context, action, state| match action {
                    Action::Toggle => State::Inactive,
                }),
            },
        );
        states.insert(
            State::Inactive,
            Transition {
                context: Some(|mut context, _action, state| {
                    context.count += 1;
                    context
                }),
                on: Some(|context, action, state| match action {
                    Action::Toggle => State::Active,
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

        assert_eq!(machine.value, State::Inactive);
        assert_eq!(machine.context.count, 0);

        machine.transition(&Action::Toggle);
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.count, 1);

        machine.transition(&Action::Toggle);
        assert_eq!(machine.value, State::Inactive);
        assert_eq!(machine.context.count, 1);

        machine.transition(&Action::Toggle);
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.count, 2);
    }
}
