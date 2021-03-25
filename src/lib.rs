//! Sigi: An organizing tool and no-frills stack database.
//!
//! Sigi contains a working CLI for stack management, and a (currently)
//! very naive on-disk stack implementation.
//!
//! The CLI and usage is documented briefly on the main GitHub project here:
//!
//! - https://github.com/hiljusti/sigi
//!
//! The "database" is currently little more than json files, and handles only
//! String values. It can work for research or small loads, but would be
//! sluggish for anything that needs to care about performance. I'm currently
//! researching approaches (and searching for existing solutions) for persistent
//! stack-based databases. (Also interested in heap and queue databases)
//!
//! Other internals are documented, but the project is early in development
//! and should be considered **unstable** at best.

// TODO: Add guidance and examples for using sigi as a library.

/// The main interface of Sigi, stack (and stack-adjacent) actions.
pub mod actions;

/// The CLI implementation.
pub mod cli;

/// The item, stack, and persistence implementation.
pub mod data;
