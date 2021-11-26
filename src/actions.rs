use crate::output::{NoiseLevel, OutputFormat};
use crate::{data, data::Item};
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

impl Action {
    pub fn data<'a>(&self) -> ActionMetadata<'a> {
        match &self {
            Peek => effect_to_old_action_metadata(effects::Peek::names, None),
            Create(_) => effect_to_old_action_metadata(
                effects::Push::names,
                Some(ActionInput::RequiredSlurpy("item")),
            ),
            Complete => effect_to_old_action_metadata(effects::Complete::names, None),
            Delete => effect_to_old_action_metadata(effects::Delete::names, None),
            DeleteAll => effect_to_old_action_metadata(effects::DeleteAll::names, None),
            List => effect_to_old_action_metadata(effects::ListAll::names, None),
            Head(_) => effect_to_old_action_metadata(
                effects::Head::names,
                Some(ActionInput::OptionalSingle("n")),
            ),
            Tail(_) => effect_to_old_action_metadata(
                effects::Tail::names,
                Some(ActionInput::OptionalSingle("n")),
            ),
            IsEmpty => effect_to_old_action_metadata(effects::IsEmpty::names, None),
            Length => effect_to_old_action_metadata(effects::Count::names, None),
            Pick(_) => pick_data(),
            Move(_) => move_data(),
            MoveAll(_) => move_all_data(),
            Next => effect_to_old_action_metadata(effects::Next::names, None),
            Swap => effect_to_old_action_metadata(effects::Swap::names, None),
            Rot => effect_to_old_action_metadata(effects::Rot::names, None),
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
            Pick(ns) => pick(self, ns),
            Move(dest) => move_item(self, dest),
            MoveAll(dest) => move_all(self, dest),
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

fn effect_to_old_action_metadata<'a>(
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

fn pick_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "pick",
        description: "Move items to the top of stack by their number",
        aliases: vec![],
        input: Some(ActionInput::RequiredSlurpy("number")),
    }
}

fn pick(command: &Command, indices: &[usize]) {
    if let Ok(stack) = data::load(&command.stack) {
        let mut stack = stack;
        let mut seen: Vec<usize> = vec![];
        seen.reserve_exact(indices.len());
        let indices: Vec<usize> = indices.iter().map(|i| stack.len() - 1 - i).rev().collect();
        for i in indices {
            if i > stack.len() {
                command.log("Pick", "ignoring out-of-bounds index");
                continue;
            } else if seen.contains(&i) {
                command.log("Pick", "ignoring duplicate index");
                continue;
            }
            let i = i - seen.iter().filter(|j| j < &&i).count();
            let picked = stack.remove(i);
            stack.push(picked);
            seen.push(i);
        }

        data::save(&command.stack, stack).unwrap();

        effects::Head {
            stack: command.stack.clone(),
            n: Some(seen.len()),
        }
        .run(command.format);
    }
}

fn move_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "move",
        description: "Move current item to another stack",
        aliases: vec![],
        input: Some(ActionInput::RequiredSingle("destination")),
    }
}

fn move_item(command: &Command, dest_stack: &str) {
    // TODO: Indirection is broken somewhere (I think I have distributed too much to each function)
    // Probably each of these functions is something like a chain of smaller bits (load, action, save)
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if let Some(item) = items.pop() {
            command.log("Move", dest_stack);
            data::save(&command.stack, items).unwrap();

            effects::Push {
                stack: dest_stack.to_string(),
                item,
            }
            .run(OutputFormat::Silent);
        }
    }
}

fn move_all_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "move-all",
        description: "Move all items to another stack",
        aliases: vec![],
        input: Some(ActionInput::RequiredSingle("destination")),
    }
}

fn move_all(command: &Command, dest_stack: &str) {
    if let Ok(src_items) = data::load(&command.stack) {
        if !src_items.is_empty() {
            command.log("Move all", dest_stack);
            if let Ok(dest_items) = data::load(dest_stack) {
                let mut dest_items = dest_items;
                for item in src_items {
                    dest_items.push(item);
                }
                data::save(dest_stack, dest_items).unwrap();
                data::save(&command.stack, vec![]).unwrap();
            } else {
                data::save(dest_stack, src_items).unwrap();
                data::save(&command.stack, vec![]).unwrap();
            }
        }
    }
}
