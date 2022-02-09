use crate::output::{OutputFormat, SimpleTableData};

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
    fn run(&self) -> Vec<SimpleTableData>;
}

pub trait LoggableStackEffect {
    fn run_logged(&self, output: OutputFormat);
} 

impl<T> LoggableStackEffect for T
where T: StackEffect {
    fn run_logged(&self, output: OutputFormat) {
        self.run().into_iter().for_each(|data| output.log(data));
    }
}

pub trait NamedEffect {
    fn names<'a>() -> EffectNames<'a>;
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
