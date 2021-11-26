use crate::output::{NoiseLevel, OutputFormat};
use crate::data::Item;
use crate::{effects, effects::StackEffect};

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

/// A stack-manipulation action.
#[derive(Clone)]
pub enum Action {
    /// Look at the most-recent item.
    Peek,
    /// Add a new item.
    Create(Item),
    /// Complete (successfully) the most-recent item.
    ///
    /// Note: Completed item is moved to a stack with the same name and the suffix `_completed`.
    Complete,
    /// Delete the most-recent item.
    ///
    /// Note: Deleted item is moved to a stack with the same name and the suffix `_deleted`.
    Delete,
    /// Delete all items.
    ///
    /// Note: Deleted items are moved to a stack with the same name and the suffix `_deleted`.
    /// If the stack name already ends in `_deleted` then it is irrecoverably deleted.
    DeleteAll,
    /// List the stack's items.
    List,
    /// List the first N stack items.
    Head(Option<usize>),
    /// List the last N stack items.
    Tail(Option<usize>),
    /// Move the specified indices to the top of stack.
    Pick(Vec<usize>),
    /// Move the current item to a different stack.
    Move(String),
    /// Move all items to a different stack.
    MoveAll(String),
    /// Count the stack's items.
    Length,
    /// Determine if the stack is empty or not.
    IsEmpty,
    /// Make the next item the most-recent item.
    /// The previously most-recent item is sent to the end of the stack.
    Next,
    /// Swap the two most-recent items.
    Swap,
    /// Rotate the three most-recent items.
    Rot,
}

use Action::*;

#[derive(Clone)]
pub enum ActionInput<'a> {
    OptionalSingle(&'a str),
    RequiredSlurpy(&'a str),
    RequiredSingle(&'a str),
}

#[derive(Clone)]
pub struct ActionMetadata<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub aliases: Vec<&'a str>,
    pub input: Option<ActionInput<'a>>,
}

impl ActionMetadata<'_> {
    fn from<'a>(
        get_names: impl Fn() -> effects::EffectNames<'a>,
        input: Option<ActionInput<'a>>,
    ) -> ActionMetadata<'a> {
        ActionMetadata {
            name: get_names().name,
            description: get_names().description,
            aliases: get_names().aliases.to_vec(),
            input,
        }
    }
}

impl Action {
    pub fn data<'a>(&self) -> ActionMetadata<'a> {
        match &self {
            Peek => ActionMetadata::from(effects::Peek::names, None),
            Create(_) => ActionMetadata::from(
                effects::Push::names,
                Some(ActionInput::RequiredSlurpy("item")),
            ),
            Complete => ActionMetadata::from(effects::Complete::names, None),
            Delete => ActionMetadata::from(effects::Delete::names, None),
            DeleteAll => ActionMetadata::from(effects::DeleteAll::names, None),
            List => ActionMetadata::from(effects::ListAll::names, None),
            Head(_) => {
                ActionMetadata::from(effects::Head::names, Some(ActionInput::OptionalSingle("n")))
            }
            Tail(_) => {
                ActionMetadata::from(effects::Tail::names, Some(ActionInput::OptionalSingle("n")))
            }
            IsEmpty => ActionMetadata::from(effects::IsEmpty::names, None),
            Length => ActionMetadata::from(effects::Count::names, None),
            Pick(_) => ActionMetadata::from(
                effects::Pick::names,
                Some(ActionInput::RequiredSlurpy("number")),
            ),
            Move(_) => ActionMetadata::from(
                effects::Move::names,
                Some(ActionInput::RequiredSingle("destination")),
            ),
            MoveAll(_) => ActionMetadata::from(
                effects::MoveAll::names,
                Some(ActionInput::RequiredSingle("destination")),
            ),
            Next => ActionMetadata::from(effects::Next::names, None),
            Swap => ActionMetadata::from(effects::Swap::names, None),
            Rot => ActionMetadata::from(effects::Rot::names, None),
        }
    }
}

/// A stack-manipulation command.
///
/// _Note: This is fairly tied to the CLI paradigm and will likely change._
pub struct Command {
    /// The action to perform.
    pub action: Action,
    /// The stack identifier.
    pub stack: String,
    /// Determines what should be printed to standard output.
    pub format: OutputFormat,
}

impl Command {
    pub fn act(&self) {
        let stack = self.stack.clone();
        let format = self.format;
        match &self.action {
            Peek => effects::Peek { stack }.run(format),
            Create(item) => {
                let item = item.clone();
                effects::Push { stack, item }.run(format)
            }
            Complete => effects::Complete { stack }.run(format),
            Delete => effects::Delete { stack }.run(format),
            DeleteAll => effects::DeleteAll { stack }.run(format),
            List => effects::ListAll { stack }.run(format),
            Tail(n) => {
                let n = n.clone();
                effects::Tail { stack, n }.run(format)
            }
            IsEmpty => effects::IsEmpty { stack }.run(format),
            Length => effects::Count { stack }.run(format),
            Head(n) => {
                let n = n.clone();
                effects::Head { stack, n }.run(format)
            }
            Pick(ns) => effects::Pick {
                stack,
                indices: ns.to_vec(),
            }
            .run(format),
            Move(dest) => effects::Move {
                stack,
                dest_stack: dest.clone(),
            }
            .run(format),
            MoveAll(dest) => effects::MoveAll {
                stack,
                dest_stack: dest.clone(),
            }
            .run(format),
            Next => effects::Next { stack }.run(format),
            Swap => effects::Swap { stack }.run(format),
            Rot => effects::Rot { stack }.run(format),
        }
    }

    pub fn log(&self, label: &str, message: &str) {
        match self.format {
            OutputFormat::Csv => println!("csv: TODO"),
            OutputFormat::Human(noise) => match noise {
                NoiseLevel::Verbose => println!("[Stack: {}] {}: {}", self.stack, label, message),
                NoiseLevel::Normal => println!("{}: {}", label, message),
                NoiseLevel::Quiet => println!("{}", message),
            },
            OutputFormat::Json => println!("json: TODO"),
            OutputFormat::Silent => {}
            OutputFormat::Tsv => println!("tsv: TODO"),
        }
    }
}
