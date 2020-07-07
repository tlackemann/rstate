#[cfg(test)]
mod tests {
    use rstate::*;
    use std::hash::Hash;

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
        entered: bool,
        count: u8,
    }

    #[test]
    fn toggle_machine() {
        let context = Context { count: 0, entered: false };
        let mut machine =
            Machine::<Action, State, Context>::new("toggle".to_string(), State::Inactive, context);

        machine.add_state(
            State::Active,
            Transition {
                context: None,
                on: Some(|_context, action, _state| match action {
                    Action::Toggle => State::Inactive,
                }),
                ..Default::default()
            },
        );

        machine.add_state(
            State::Inactive,
            Transition {
                context: Some(|mut context, _action, _state| {
                    context.count += 1;
                    context
                }),
                on: Some(|_context, action, _state| match action {
                    Action::Toggle => State::Active,
                }),
                ..Default::default()
            },
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
