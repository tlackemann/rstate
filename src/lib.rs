//! A state machine library for Rust, inspired by [xstate](https://github.com/davidkpiano/xstate).

pub mod machine;
pub mod parallel;

pub use self::machine::*;
pub use self::parallel::*;
