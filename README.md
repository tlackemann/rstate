# rstate

A Rust library for creating and executing statecharts. Heavily inspired by [xstate](https://github.com/davidkpiano/xstate).

## Usage

State machines can be created by defining States, Actions, and Transitions.

### Running Examples

```bash

cargo run --example <example-name>
```

You can find relevant examples in the `examples/` folder.

### Basic Increment Example

```rust
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

let context = Context { count: 0 };
let mut machine = Machine::<Action, State, Context>::new(
    "increment".to_string(),
    State::Active,
    context,
);

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
    },
);

machine.add_state(
    State::Done,
    Transition {
        on: None,
        context: None,
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
```

### Parallel Machine Example

```rust
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

let mut bold_machine = Machine::<Action, State, Context>::new(
    "bold".to_string(),
    State::Bold(ToggleState::Off),
    context,
);
bold_machine.add_state(
    State::Bold(ToggleState::Off),
    Transition {
        context: None,
        on: Some(|_context, action, state| match action {
            Action::ToggleBold => State::Bold(ToggleState::On),
            _ => state,
        }),
    },
);
bold_machine.add_state(
    State::Bold(ToggleState::On),
    Transition {
        context: None,
        on: Some(|_context, action, state| match action {
            Action::ToggleBold => State::Bold(ToggleState::Off),
            _ => state,
        }),
    },
);

let mut italics_machine = Machine::<Action, State, Context>::new(
    "italics".to_string(),
    State::Italics(ToggleState::Off),
    context,
);
italics_machine.add_state(
    State::Italics(ToggleState::Off),
    Transition {
        context: None,
        on: Some(|_context, action, state| match action {
            Action::ToggleItalics => State::Italics(ToggleState::On),
            _ => state,
        }),
    },
);
italics_machine.add_state(
    State::Italics(ToggleState::On),
    Transition {
        context: None,
        on: Some(|_context, action, state| match action {
            Action::ToggleItalics => State::Italics(ToggleState::Off),
            _ => state,
        }),
    },
);

let mut underline_machine = Machine::<Action, State, Context>::new(
    "underline".to_string(),
    State::Underline(ToggleState::Off),
    context,
);
underline_machine.add_state(
    State::Underline(ToggleState::Off),
    Transition {
        context: None,
        on: Some(|_context, action, state| match action {
            Action::ToggleUnderline => State::Underline(ToggleState::On),
            _ => state,
        }),
    },
);
underline_machine.add_state(
    State::Underline(ToggleState::On),
    Transition {
        context: None,
        on: Some(|_context, action, state| match action {
            Action::ToggleUnderline => State::Underline(ToggleState::Off),
            _ => state,
        }),
    },
);

let mut list_machine = Machine::<Action, State, Context>::new(
    "list".to_string(),
    State::List(ListState::None),
    context,
);
list_machine.add_state(
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
list_machine.add_state(
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
list_machine.add_state(
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
```

## License

MIT
