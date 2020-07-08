#[cfg(test)]
mod tests {
    use rstate::*;
    use std::hash::Hash;

    #[derive(Copy, Clone, Debug)]
    enum Action {
        Start,
        Tick,
        Finish,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum State {
        Idle,
        Active,
        Finished,
    }

    #[derive(Debug, Clone, Copy)]
    struct Context {
        dirty_count: u8,
        tick: u8,
    }

    #[test]
    fn toggle_machine() {
        let context = Context {
            tick: 0,
            dirty_count: 0,
        };
        let mut machine =
            Machine::<Action, State, Context>::new("tick".to_string(), State::Idle, context);

        machine.add_state(
            State::Idle,
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::Start => State::Active,
                    _ => state,
                }),
                ..Default::default()
            },
        );

        machine.add_state(
            State::Active,
            Transition {
                context: Some(|mut context, action, _state| match action {
                    Action::Tick => {
                        context.tick += 1;
                        context
                    }
                    _ => context,
                }),
                on: Some(|_context, action, state| match action {
                    Action::Finish => State::Finished,
                    _ => state,
                }),
                on_entry: Some(|mut context, _action, _state| {
                    context.dirty_count += 1;
                    context
                }),
                on_exit: Some(|mut context, _action, _state| {
                    context.dirty_count = 0;
                    context
                }),
                ..Default::default()
            },
        );

        machine.add_state(
            State::Finished,
            Transition {
                ..Default::default()
            },
        );

        assert_eq!(machine.value, State::Idle);
        assert_eq!(machine.context.tick, 0);
        assert_eq!(machine.context.dirty_count, 0);

        machine.transition(&Action::Start);
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.tick, 0);
        assert_eq!(machine.context.dirty_count, 1);

        machine.transition(&Action::Tick);
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.tick, 1);
        assert_eq!(machine.context.dirty_count, 1);

        machine.transition(&Action::Tick);
        assert_eq!(machine.value, State::Active);
        assert_eq!(machine.context.tick, 2);
        assert_eq!(machine.context.dirty_count, 1);

        machine.transition(&Action::Finish);
        assert_eq!(machine.value, State::Finished);
        assert_eq!(machine.context.tick, 2);
        assert_eq!(machine.context.dirty_count, 0);
    }
}
