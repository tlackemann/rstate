#[cfg(test)]
mod tests {
    use rstate::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn toggle_machine() {
        #[derive(Copy, Clone, Debug)]
        enum Action {
            Bullets,
            Numbers,
            None,
            ToggleBold,
            ToggleItalics,
            ToggleUnderline,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum ToggleState {
            On,
            Off,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum ListState {
            None,
            Numbers,
            Bullets,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum State {
            Bold(ToggleState),
            Italics(ToggleState),
            Underline(ToggleState),
            List(ListState),
        }

        #[derive(Debug, Clone, Copy)]
        struct Context {}

        let context = Context {};

        let mut bold_states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
        bold_states.insert(
            State::Bold(ToggleState::Off),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::ToggleBold => State::Bold(ToggleState::On),
                    _ => state,
                }),
            },
        );
        bold_states.insert(
            State::Bold(ToggleState::On),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::ToggleBold => State::Bold(ToggleState::Off),
                    _ => state,
                }),
            },
        );

        let mut italics_states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
        italics_states.insert(
            State::Italics(ToggleState::Off),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::ToggleItalics => State::Italics(ToggleState::On),
                    _ => state,
                }),
            },
        );
        italics_states.insert(
            State::Italics(ToggleState::On),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::ToggleItalics => State::Italics(ToggleState::Off),
                    _ => state,
                }),
            },
        );

        let mut underline_states: HashMap<State, Transition<Action, State, Context>> =
            HashMap::new();
        underline_states.insert(
            State::Underline(ToggleState::Off),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::ToggleUnderline => State::Underline(ToggleState::On),
                    _ => state,
                }),
            },
        );
        underline_states.insert(
            State::Underline(ToggleState::On),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::ToggleUnderline => State::Underline(ToggleState::Off),
                    _ => state,
                }),
            },
        );

        let mut list_states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
        list_states.insert(
            State::List(ListState::None),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::Bullets => State::List(ListState::Bullets),
                    Action::Numbers => State::List(ListState::Numbers),
                    Action::None => State::List(ListState::None),
                    _ => state,
                }),
            },
        );
        list_states.insert(
            State::List(ListState::Numbers),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::Bullets => State::List(ListState::Bullets),
                    Action::Numbers => State::List(ListState::Numbers),
                    Action::None => State::List(ListState::None),
                    _ => state,
                }),
            },
        );
        list_states.insert(
            State::List(ListState::Bullets),
            Transition {
                context: None,
                on: Some(|_context, action, state| match action {
                    Action::Bullets => State::List(ListState::Bullets),
                    Action::Numbers => State::List(ListState::Numbers),
                    Action::None => State::List(ListState::None),
                    _ => state,
                }),
            },
        );

        let bold_machine = Machine::<Action, State, Context>::new(
            "bold".to_string(),
            State::Bold(ToggleState::Off),
            bold_states,
            context,
        );
        let italics_machine = Machine::<Action, State, Context>::new(
            "italics".to_string(),
            State::Italics(ToggleState::Off),
            italics_states,
            context,
        );
        let underline_machine = Machine::<Action, State, Context>::new(
            "underline".to_string(),
            State::Underline(ToggleState::Off),
            underline_states,
            context,
        );
        let list_machine = Machine::<Action, State, Context>::new(
            "list".to_string(),
            State::List(ListState::None),
            list_states,
            context,
        );

        let mut machine = ParallelMachine::<Action, State, Context>::new(
            "parallel".to_string(),
            vec![
                bold_machine,
                italics_machine,
                underline_machine,
                list_machine,
            ],
        );

        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::Off),
                State::Underline(ToggleState::Off),
                State::List(ListState::None),
            ]
        );

        machine.transition(&Action::Numbers);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::Off),
                State::Underline(ToggleState::Off),
                State::List(ListState::Numbers),
            ]
        );

        machine.transition(&Action::ToggleBold);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::On),
                State::Italics(ToggleState::Off),
                State::Underline(ToggleState::Off),
                State::List(ListState::Numbers),
            ]
        );

        machine.transition(&Action::ToggleBold);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::Off),
                State::Underline(ToggleState::Off),
                State::List(ListState::Numbers),
            ]
        );

        machine.transition(&Action::ToggleItalics);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::On),
                State::Underline(ToggleState::Off),
                State::List(ListState::Numbers),
            ]
        );

        machine.transition(&Action::Bullets);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::On),
                State::Underline(ToggleState::Off),
                State::List(ListState::Bullets),
            ]
        );

        machine.transition(&Action::None);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::On),
                State::Underline(ToggleState::Off),
                State::List(ListState::None),
            ]
        );

        machine.transition(&Action::ToggleUnderline);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::On),
                State::Underline(ToggleState::On),
                State::List(ListState::None),
            ]
        );

        machine.transition(&Action::ToggleUnderline);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::On),
                State::Underline(ToggleState::Off),
                State::List(ListState::None),
            ]
        );

        machine.transition(&Action::ToggleItalics);
        assert_eq!(
            machine.value,
            vec![
                State::Bold(ToggleState::Off),
                State::Italics(ToggleState::Off),
                State::Underline(ToggleState::Off),
                State::List(ListState::None),
            ]
        );
    }
}
