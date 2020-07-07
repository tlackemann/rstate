# rstate

A Rust library for creating and executing statecharts. Heavily inspired by [xstate](https://github.com/davidkpiano/xstate).

## Usage

State machines can be created by defining States, Actions, and Transitions. You
can also optionally define a Context for your machine.

```rust
/// Define available states in this machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
  Active,
  Inactive,
}

/// Define actions for this machine
#[derive(Debug, PartialEq, Eq, Hash)]
enum Action {
  Toggle,
}

// Define context (global state) for the machine
#[derive(Debug, Clone, Copy)]
struct Context {
  count: u8,
}

/// Define transitions between states from actions
let mut active_states: HashMap<Action, Transition<State>> = HashMap::new();
active_states.insert(
  Action::Toggle,
  Transition {
    action: None,
    state: State::Inactive,
  },
);

let mut inactive_states: HashMap<Action, Transition<State>> = HashMap::new();
inactive_states.insert(
  Action::Toggle,
  Transition {
    action: Some(|mut context| { context.count += 1; context }),
    state: State::Active,
  },
);

let mut states: HashMap<State, HashMap<Action, Transition<State>>> = HashMap::new();
states.insert(State::Active, active_states);
states.insert(State::Inactive, inactive_states);

/// Create the machine
let mut machine = Machine::<Action, State>::new("toggle".to_string(), State::Inactive, states);

// Start in the inactive state
assert_eq!(machine.current, State::Inactive);
assert_eq!(machine.context.count, 0);

// Toggle active
machine.send(Action::Toggle);
assert_eq!(machine.current, State::Active);
assert_eq!(machine.context.count, 1);

// Toggle inactive
machine.send(Action::Toggle);
assert_eq!(machine.current, State::Inactive);
assert_eq!(machine.context.count, 1);
```
