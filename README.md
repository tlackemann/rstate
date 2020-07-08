# rstate

A Rust library for creating and interpreting statecharts.

Although not ready at the moment, rstate aims to be a complete [SCXML](https://www.w3.org/TR/scxml/#CoreIntroduction)-compliant
library for Rust.


## Usage

State machines can be created by defining States, Actions, and Transitions.

### Running Examples

```bash

cargo run --example <example-name>
```

You can find relevant examples in the `examples/` folder.

### Basic Increment Example

```rust
// Create transition actions the state chart can receive
#[derive(Debug, Copy, Clone)]
enum Action {
    Increment(u8),
    Decrement(u8),
    Finished,
}

// Create valid states the state machine can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Active,
    Done,
}

// Create global state the state machine can hold
#[derive(Debug, Clone, Copy)]
struct Context {
    count: u8,
}

// Create the state machine
let mut machine = Machine::<Action, State, Context>::new(
    "increment".to_string(),
    State::Active,
    Context { count: 0 },
);

// Add a state node
machine.add_state(
    State::Active,
    Transition {
        // The "on" event behaves similar to a reducer
        // When an action is sent to the machine, we interpret the next state by returning it
        on: Some(|_context, action, _state| match action {
            Action::Finished => State::Done,
            _ => State::Active,
        }),
        // Context behaves similar to "on" and like a reducer
        // Return the next context based on the action
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

## License

MIT
