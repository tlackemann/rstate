//! A state machine library for Rust, inspired by [xstate](https://github.com/davidkpiano/xstate).

pub mod machine;
pub mod parallel;
pub mod history;

pub use self::machine::*;
pub use self::parallel::*;
pub use self::history::*;
