# rstate

A Rust library for creating and executing statecharts. Heavily inspired by [xstate](https://github.com/davidkpiano/xstate).

## Usage

State machines can be created by defining States, Actions, and Transitions.

### Basic Increment Example

```rust
// Define actions the state machine can perform
#[derive(Debug, Copy, Clone)]
enum Action {
    Increment(u8),
    Decrement(u8),
}

// Define states the state machine can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Active,
}

// Define global state the state machine can store
#[derive(Debug, Clone, Copy)]
struct Context {
    count: u8,
}

// Define transitions between states
// The current state and context of the machine is handled like a reducer
// Return the next state and context to update the machine
let mut states: HashMap<State, Transition<Action, State, Context>> = HashMap::new();
states.insert(
    State::Active,
    Transition {
        state: Some(|_state, _action| State::Active),
        context: Some(|mut context, action| {
            match action {
                Action::Increment(val) => context.count += val,
                Action::Decrement(val) => context.count -= val,
            }
            context
        }),
    },
);

// Create the machine
let mut machine = Machine::<Action, State, Context>::new(
    "increment".to_string(),
    State::Active,
    states,
    Context { count: 0 },
);

assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 0);

machine.send(&Action::Increment(1));
assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 1);

machine.send(&Action::Decrement(1));
assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 0);

machine.send(&Action::Increment(5));
assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 5);
```

### Basic Toggle Example

```rust
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
        state: Some(|_state, action| {
            match action {
                Action::Toggle => State::Inactive
            }
        }),
    },
);
states.insert(
    State::Inactive,
    Transition {
        context: Some(|mut context, _action| {
            context.count += 1;
            context
        }),
        state: Some(|_state, action| {
            match action {
                Action::Toggle => State::Active
            }
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

assert_eq!(machine.current, State::Inactive);
assert_eq!(machine.context.count, 0);

machine.send(&Action::Toggle);
assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 1);

machine.send(&Action::Toggle);
assert_eq!(machine.current, State::Inactive);
assert_eq!(machine.context.count, 1);

machine.send(&Action::Toggle);
assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 2);
```

## License

MIT
