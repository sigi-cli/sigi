use crate::{data, data::Item, data::Stack};
use chrono::{DateTime, Local};

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

const COMPLETED_SUFFIX: &str = "_completed";
const DELETED_SUFFIX: &str = "_deleted";

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
    // TODO: Something's screwy with this interface.
    //       I think what I really want is a trait, and let each action implement it.
    pub fn data<'a>(&self) -> ActionMetadata<'a> {
        match &self {
            Peek => peek_data(),
            Create(_) => create_data(),
            Complete => complete_data(),
            Delete => delete_data(),
            DeleteAll => delete_all_data(),
            List => list_data(),
            Head(_) => head_data(),
            Tail(_) => tail_data(),
            Pick(_) => pick_data(),
            Move(_) => move_data(),
            MoveAll(_) => move_all_data(),
            IsEmpty => is_empty_data(),
            Length => length_data(),
            Next => next_data(),
            Swap => swap_data(),
            Rot => rot_data(),
        }
    }
}

/// How much noise (verbosity) should be used when printing to standard output.
#[derive(Clone, Copy, PartialEq)]
pub enum NoiseLevel {
    Verbose,
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
            Head(n) => head(self, n),
            Tail(n) => tail(self, n),
            Pick(ns) => pick(self, ns),
            Move(dest) => move_item(self, dest),
            MoveAll(dest) => move_all(self, dest),
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
            NoiseLevel::Verbose => println!("[Stack: {}] {}: {}", self.stack, label, message),
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
        description: "Show the current item",
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
        description: "Move the current item to \"<STACK>_completed\"",
        aliases: vec!["done", "finish", "fulfill"],
        input: None,
    }
}

fn complete(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if let Some(completed) = items.pop() {
            command.log("Completed", &completed.name);

            let mut completed = completed;
            completed.succeeded = Some(Local::now());

            let create_command = Command {
                action: Create(completed.clone()),
                noise: NoiseLevel::Silent,
                stack: command.stack.clone() + COMPLETED_SUFFIX,
            };

            create(&create_command, &completed);
        }
        data::save(&command.stack, items).unwrap();
        peek(command);
    }
}

fn delete_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "delete",
        description: "Move the current item to \"<STACK>_deleted\"",
        aliases: vec!["pop", "remove", "cancel", "drop"],
        input: None,
    }
}

fn delete(command: &Command) {
    if let Ok(items) = data::load(&command.stack) {
        let mut items = items;
        if let Some(deleted) = items.pop() {
            command.log("Deleted", &deleted.name);

            let mut deleted = deleted;
            deleted.failed = Some(Local::now());

            let create_command = Command {
                action: Create(deleted.clone()),
                noise: NoiseLevel::Silent,
                stack: command.stack.clone() + DELETED_SUFFIX,
            };

            create(&create_command, &deleted);
        }
        data::save(&command.stack, items).unwrap();
        peek(command);
    }
}

fn delete_all_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "delete-all",
        description: "Move all items to \"<STACK>_deleted\"",
        aliases: vec!["purge", "pop-all", "remove-all", "cancel-all", "drop-all"],
        input: None,
    }
}

fn delete_all(command: &Command) {
    if !command.stack.ends_with("_deleted") {
        if let Ok(stack) = data::load(&command.stack) {
            data::save(&(command.stack.clone() + DELETED_SUFFIX), stack).unwrap();
        }
    }
    data::save(&command.stack, vec![]).unwrap()
}

fn list_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "list",
        description: "List all items",
        aliases: vec!["ls", "snoop", "show", "all"],
        input: None,
    }
}

fn list(command: &Command) {
    if let NoiseLevel::Silent = command.noise {
        return;
    }

    if let Ok(stack) = data::load(&command.stack) {
        let len = stack.len();
        list_range(command, stack, 0, len);
    }
}

fn list_range(command: &Command, stack: Stack, from: usize, n: usize) {
    // Checks for NoiseLevel::Silent should happen in calling functions. (To avoid potentially costly load of stack)

    if n == 0 {
        return;
    }

    if stack.is_empty() {
        command.log("None", "");
        return;
    }

    let mut stack = stack;
    stack.reverse();
    if NoiseLevel::Quiet == command.noise {
        stack.iter().for_each(|item| println!("{}", item.name));
        return;
    }

    let description_of = |item: &Item| match command.noise {
        NoiseLevel::Verbose => {
            let name = &item.name;
            let created = format_time_for_humans(item.created);
            let succeeded = item
                .succeeded
                .map(format_time_for_humans)
                .unwrap_or_else(|| "N/A".to_string());
            let deleted = item
                .failed
                .map(format_time_for_humans)
                .unwrap_or_else(|| "N/A".to_string());
            format!(
                "{} (Created: {} | Completed: {} | Deleted: {})",
                name, created, succeeded, deleted
            )
        }
        _ => item.name.to_string(),
    };

    let (start, n) = if from == 0 {
        println!("Now: {}", description_of(&stack[0]));
        (1, n - 1)
    } else {
        (from, n)
    };

    stack
        .iter()
        .enumerate()
        .skip(start)
        .take(n)
        .for_each(|(n, item)| println!("{: >3}: {}", n, description_of(item)));
}

fn head_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "head",
        description: "List the first N items",
        aliases: vec!["top", "first"],
        input: Some(ActionInput::OptionalSingle("n")),
    }
}

fn head(command: &Command, n: &Option<usize>) {
    if let NoiseLevel::Silent = command.noise {
        return;
    }

    if let Ok(stack) = data::load(&command.stack) {
        let n = n.unwrap_or(10);
        list_range(command, stack, 0, n);
    }
}

fn tail_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "tail",
        description: "List the last N items",
        aliases: vec!["bottom", "last"],
        input: Some(ActionInput::OptionalSingle("n")),
    }
}

fn tail(command: &Command, n: &Option<usize>) {
    if let NoiseLevel::Silent = command.noise {
        return;
    }

    if let Ok(stack) = data::load(&command.stack) {
        let n = n.unwrap_or(10);
        if n >= stack.len() {
            list(command)
        } else {
            let start = stack.len() - n;
            list_range(command, stack, start, n);
        };
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

        let picked_n = Some(seen.len());
        let head_cmd = Command {
            action: Action::Head(picked_n),
            noise: command.noise,
            stack: command.stack.clone(),
        };
        head(&head_cmd, &picked_n);
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

            let command = Command {
                action: Create(item.clone()),
                noise: NoiseLevel::Silent,
                stack: String::from(dest_stack),
            };
            create(&command, &item);
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

fn is_empty_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "is-empty",
        description: "\"true\" if stack has no items, \"false\" otherwise",
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
            std::process::exit(1)
        }
    }
}

fn length_data<'a>() -> ActionMetadata<'a> {
    ActionMetadata {
        name: "length",
        description: "Print the stack's length",
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
        description: "Cycle to the next item; the current item becomes last",
        aliases: vec!["later", "cycle", "bury"],
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
        description: "Swap the two most-current items",
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
        head(command, &Some(2));
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
        head(command, &Some(3));
    }
}

fn format_time_for_humans(dt: DateTime<Local>) -> String {
    // TODO: Does this work for all locales?
    dt.to_rfc2822()
}
