#[cfg(test)]
mod tests {
    use rstate::*;
    use std::hash::Hash;

    #[test]
    fn toggle_machine() {
        #[derive(Copy, Clone, Debug)]
        enum Action {
            Timer,
            PedestrianTimer,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum Red {
            Wait,
            Walk,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum State {
            Green,
            Yellow,
            Red(Red),
        }

        #[derive(Debug, Clone, Copy)]
        struct Context {
            timer: u8,
        }

        let context = Context { timer: 3 };
        let mut machine = Machine::<Action, State, Context>::new(
            "hierarchical-light".to_string(),
            State::Green,
            context,
        );
        machine.add_state(
            State::Green,
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::Timer => State::Yellow,
                    _ => state,
                }),
            },
        );
        machine.add_state(
            State::Yellow,
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::Timer => State::Red(Red::Wait),
                    _ => state,
                }),
            },
        );
        machine.add_state(
            State::Red(Red::Wait),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::PedestrianTimer => State::Red(Red::Walk),
                    _ => state,
                }),
            },
        );
        machine.add_state(
            State::Red(Red::Walk),
            Transition {
                context: Some(|mut context, action, _state| {
                    match action {
                        Action::PedestrianTimer => context.timer -= 1,
                        _ => {}
                    };
                    context
                }),
                on: Some(|context, action, state| match action {
                    Action::PedestrianTimer => State::Red(Red::Walk),
                    Action::Timer => {
                        if context.timer == 0 {
                            State::Green
                        } else {
                            state
                        }
                    }
                }),
            },
        );

        assert_eq!(machine.value, State::Green);
        machine.transition(&Action::Timer);
        assert_eq!(machine.value, State::Yellow);
        machine.transition(&Action::Timer);
        assert_eq!(machine.value, State::Red(Red::Wait));

        // does not change state
        machine.transition(&Action::Timer);
        assert_eq!(machine.value, State::Red(Red::Wait));

        // countdown the timer
        machine.transition(&Action::PedestrianTimer);
        machine.transition(&Action::Timer); // does not change to green
        assert_eq!(machine.value, State::Red(Red::Walk));
        assert_eq!(machine.context.timer, 3);
        machine.transition(&Action::PedestrianTimer);
        machine.transition(&Action::Timer); // does not change to green
        assert_eq!(machine.value, State::Red(Red::Walk));
        assert_eq!(machine.context.timer, 2);
        machine.transition(&Action::PedestrianTimer);
        machine.transition(&Action::Timer); // does not change to green
        assert_eq!(machine.value, State::Red(Red::Walk));
        assert_eq!(machine.context.timer, 1);
        machine.transition(&Action::PedestrianTimer);
        machine.transition(&Action::Timer); // now ready to change to green
        assert_eq!(machine.value, State::Green);
    }
}
