use crate::{data, data::Item};

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

/// A stack-manipulation action.
pub enum Action {
    /// Look at the most-recent item.
    Peek,
    /// Add a new item.
    Create(Item),
    /// Complete (successfully) the most-recent item.
    ///
    /// _Note: This currently does nothing different from `Delete`.
    /// In future versions, sigi is planned to have a history of
    /// your completed items._
    Complete,
    /// Delete the most-recent item.
    Delete,
    /// Delete all items.
    DeleteAll,
    /// List the stack's items.
    List,
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

pub enum ActionInput<'a> {
    RequiredSlurpy(&'a str),
}

pub struct ActionMetadata<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub aliases: Vec<&'a str>,
    pub input: Option<ActionInput<'a>>,
}

impl Action {
    pub fn data<'a>(&self) -> ActionMetadata<'a> {
        match &self {
            Peek => peek_data(),
            Create(_) => create_data(),
            Complete => complete_data(),
            Delete => delete_data(),
            DeleteAll => delete_all_data(),
            List => list_data(),
            IsEmpty => is_empty_data(),
            Length => length_data(),
            Next => next_data(),
            Swap => swap_data(),
            Rot => rot_data(),
        }
    }
}

/// How much noise (verbosity) should be used when printing to standard output.
pub enum NoiseLevel {
    Normal,
    Quiet,
    Silent,
}

/// A stack-manipulation command.
///
/// _Note: This is fairly tied to the CLI paradigm and will likely change._
pub struct Command {
    /// The action to perform.
    pub action: Action,
    /// The stack identifier.
    pub stack: String,
    /// Determines how much should be printed to standard output.
    pub noise: NoiseLevel,
}

impl Command {
    pub fn act(&self) {
        match &self.action {
            Peek => peek(self),
            Create(item) => create(self, item),
            Complete => complete(self),
            Delete => delete(self),
            DeleteAll => delete_all(self),
            List => list(self),
            IsEmpty => is_empty(self),
            Length => length(self),
            Next => next(self),
            Swap => swap(self),
            Rot => rot(self),
        }
    }

    // TODO: Actually use a logger. (Are there any that don't explode binary size?)
    pub fn log(&self, label: &str, message: &str) {
        match self.noise {
            NoiseLevel::Normal => println!("{}: {}", label, message),
            NoiseLevel::Quiet => println!("{}", message),
            NoiseLevel::Silent => {}
        }
    }
}

// TODO: Make command processors return `Result<(), Error>`. Many error cases are not covered (e.g. create with no content)

fn peek_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "peek",
        description: "Peek at the current item",
        aliases: vec!["show"],
        input: None,
    }
}

fn peek(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        if !items.is_empty() {
            command.log("Now", &items.last().unwrap().name)
        }
    }
}

fn create_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "create",
        description: "Create a new item",
        aliases: vec!["push", "add", "do", "start", "new"],
        input: Some(ActionInput::RequiredSlurpy("item")),
    }
}

fn create(command: &Command, item: &Item) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        items.push(item.clone());
        data::save(&command.stack, items).unwrap();
        command.log("Created", &item.name);
    } else {
        data::save(&command.stack, vec![item.clone()]).unwrap();
        command.log("Created", &item.name);
    }
}

fn complete_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "complete",
        description: "Mark the current item as successfully completed",
        aliases: vec!["done", "finish", "fulfill"],
        input: None,
    }
}

fn complete(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if let Some(completed) = items.pop() {
            command.log("Completed", &completed.name);
            // TODO: Archive instead of delete. (update, save somewhere recoverable)
            // TODO: Might be nice to have a "history" Action for viewing these.
        }
        data::save(&command.stack, items).unwrap();
    }
}

fn delete_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "delete",
        description: "Delete the current item",
        aliases: vec!["pop", "remove", "cancel", "drop", "abandon", "retire"],
        input: None,
    }
}

fn delete(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if let Some(deleted) = items.pop() {
            command.log("Deleted", &deleted.name);
            // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
            // Might allow an easy "undo" or "undelete"; would need a "purge" idea
            // TODO: Might be nice to have a "history" Action for viewing these
        }
        data::save(&command.stack, items).unwrap();
    }
}

fn delete_all_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "delete-all",
        description: "Delete all items",
        aliases: vec![
            "purge",
            "pop-all",
            "remove-all",
            "cancel-all",
            "drop-all",
            "abandon-all",
            "retire-all",
        ],
        input: None,
    }
}

fn delete_all(command: &Command) {
    data::save(&command.stack, vec![]).unwrap()
}

fn list_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "list",
        description: "List all items",
        aliases: vec!["show", "all"],
        input: None,
    }
}

fn list(command: &Command) {
    if let NoiseLevel::Silent = command.noise {
        return;
    }
    // TODO: Think on this. This limits practical size, but needs a change to the
    // save/load format and/or algorithms to scale.
    if let Ok(items) = data::load(&command.stack) {
        if !items.is_empty() {
            let mut items = items;
            items.reverse();
            match command.noise {
                NoiseLevel::Quiet => items.iter().for_each(|item| println!("{}", item.name)),
                _ => {
                    println!("Now: {}", items[0].name);
                    items
                        .iter()
                        .enumerate()
                        .skip(1)
                        .for_each(|(n, item)| println!("{: >3}: {}", n, item.name))
                }
            }
        }
    }
}

fn is_empty_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "is-empty",
        description: "Determine if stack is empty",
        aliases: vec!["empty"],
        input: None,
    }
}

fn is_empty(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let is_empty = items.is_empty();
        command.log("Empty", &is_empty.to_string());
        if !is_empty {
            // TODO: This would be better as an Err, once everything returns Result
            panic!()
        }
    }
}

fn length_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "length",
        description: "Count all items",
        aliases: vec!["count", "size"],
        input: None,
    }
}

fn length(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        command.log("Items", &items.len().to_string())
    }
}

fn next_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "next",
        description: "Move the next item to current, and moves current to last",
        aliases: vec!["later", "punt", "bury"],
        input: None,
    }
}

fn next(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if items.is_empty() {
            return;
        }
        let to_the_back = items.pop().unwrap();
        items.insert(0, to_the_back);

        data::save(&command.stack, items).unwrap();
        peek(command)
    }
}

fn swap_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "swap",
        description: "Swap the two most current items",
        aliases: vec![],
        input: None,
    }
}

fn swap(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if items.len() < 2 {
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        items.push(a);
        items.push(b);

        data::save(&command.stack, items).unwrap();
        peek(command)
    }
}

fn rot_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "rot",
        description: "Rotate the three most-current items",
        aliases: vec!["rotate"],
        input: None,
    }
}

fn rot(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if items.len() < 3 {
            swap(command);
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        let c = items.pop().unwrap();
        items.push(a);
        items.push(c);
        items.push(b);

        data::save(&command.stack, items).unwrap();
        peek(command)
    }
}
