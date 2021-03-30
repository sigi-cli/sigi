use crate::{data, data::Item};
use chrono::Local;

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

/// A stack-manipulation action.
pub enum Action<A> {
    /// Look at the most-recent item.
    Peek,
    /// Add a new item.
    Create(A),
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
    pub action: Action<String>,
    /// The stack identifier.
    ///
    /// _Note: This member name is likely to change in the future._
    pub topic: String,
    /// Determines how much should be printed to standard output.
    pub noise: NoiseLevel,
}

impl Command {
    pub fn act(&self) {
        match &self.action {
            Peek => peek(self),
            Create(name) => create(self, name),
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

// TODO: Return Result<(), Error> - some error cases are not covered (e.g. create with no content)

fn create(command: &Command, name: &str) {
    let item = Item {
        name: name.to_string(),
        created: Local::now(),
        succeeded: None,
        failed: None,
    };
    if let Ok(items) = data::load(command) {
        let mut items = items;
        items.push(item);
        data::save(command, items).unwrap();
        command.log("Created", name);
    } else {
        data::save(command, vec![item]).unwrap();
        command.log("Created", name);
    }
}

fn complete(command: &Command) {
    if let Ok(items) = data::load(command) {
        let mut items = items;
        if let Some(completed) = items.pop() {
            command.log("Completed", &completed.name);
            // TODO: Archive instead of delete. (update, save somewhere recoverable)
            // TODO: Might be nice to have a "history" Action for viewing these.
        }
        data::save(command, items).unwrap();
    }
}

fn delete(command: &Command) {
    if let Ok(items) = data::load(command) {
        let mut items = items;
        if let Some(deleted) = items.pop() {
            command.log("Deleted", &deleted.name);
            // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
            // Might allow an easy "undo" or "undelete"; would need a "purge" idea
            // TODO: Might be nice to have a "history" Action for viewing these
        }
        data::save(command, items).unwrap();
    }
}

fn delete_all(command: &Command) {
    data::save(command, vec![]).unwrap()
}

fn list(command: &Command) {
    if let NoiseLevel::Silent = command.noise {
        return;
    }
    // TODO: Think on this. This limits practical size, but needs a change to the
    // save/load format and/or algorithms to scale.
    if let Ok(items) = data::load(command) {
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

fn is_empty(command: &Command) {
    if let Ok(items) = data::load(command) {
        let is_empty = items.is_empty();
        command.log("Empty", &is_empty.to_string());
        if !is_empty {
            // TODO: This would be better as an Err, once everything returns Result
            panic!()
        }
    }
}

fn length(command: &Command) {
    if let Ok(items) = data::load(command) {
        command.log("Items", &items.len().to_string())
    }
}

fn peek(command: &Command) {
    if let Ok(items) = data::load(command) {
        if !items.is_empty() {
            command.log("Now", &items.last().unwrap().name)
        }
    }
}

fn swap(command: &Command) {
    if let Ok(items) = data::load(command) {
        let mut items = items;
        if items.len() < 2 {
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        items.push(a);
        items.push(b);

        data::save(command, items).unwrap();
        peek(command)
    }
}

fn rot(command: &Command) {
    if let Ok(items) = data::load(command) {
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

        data::save(command, items).unwrap();
        peek(command)
    }
}

fn next(command: &Command) {
    if let Ok(items) = data::load(command) {
        let mut items = items;
        if items.is_empty() {
            return;
        }
        let to_the_back = items.pop().unwrap();
        items.insert(0, to_the_back);

        data::save(command, items).unwrap();
        peek(command)
    }
}
