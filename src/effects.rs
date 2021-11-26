use crate::output::OutputFormat;

pub mod lifecycle;
pub use lifecycle::*;
pub mod views;
pub use views::*;
pub mod shuffle;
pub use shuffle::*;
pub mod housekeeping;
pub use housekeeping::*;

const HISTORY_SUFFIX: &str = "_history";

pub trait StackEffect {
    fn names<'a>() -> EffectNames<'a>;
    fn run(&self, output: OutputFormat);
}

pub struct EffectNames<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub aliases: &'a [&'a str],
    pub input: EffectInput<'a>,
}

pub enum EffectInput<'a> {
    NoInput,
    OptionalSingle(&'a str),
    RequiredSingle(&'a str),
    RequiredSlurpy(&'a str),
}

impl<'a> EffectInput<'a> {
    pub fn arg_name(&self) -> &'a str {
        match self {
            EffectInput::NoInput => "NONE",
            EffectInput::OptionalSingle(name) => name,
            EffectInput::RequiredSingle(name) => name,
            EffectInput::RequiredSlurpy(name) => name,
        }
    }
}

// ===== Helper functions =====

fn stack_history_of(stack: &str) -> String {
    stack.to_string() + HISTORY_SUFFIX
}
