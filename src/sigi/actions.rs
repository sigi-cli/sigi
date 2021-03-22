use crate::sigi::data;
use crate::sigi::items::Item;
use chrono::Local;

pub enum Action {
    Peek,
    Create(String),
    Complete,
    Delete,
    DeleteAll,
    List,
    ListAll,
    Length,
    IsEmpty,
    Next,
    Swap,
    Rot,
}

use Action::*;

pub struct Command {
    pub action: Action,
    pub topic: String,
    pub quiet: bool,
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
            ListAll => list_all(self),
            IsEmpty => is_empty(self),
            Length => length(self),
            Next => next(self),
            Swap => swap(self),
            Rot => rot(self),
        }
    }
}
// TODO: Refactor. The repetition in function signatures suggests struct { &str, Option<ArgMatches> }
// TODO: Return Result<(), Error> - some error cases are not covered (e.g. create with no content)

fn create(command: &Command, name: &str) {
    println!("{}{}", if command.quiet { "" } else { "Creating: " }, name);
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
    } else {
        data::save(command, vec![item]).unwrap();
    }
}

fn complete(command: &Command) {
    if let Ok(items) = data::load(command) {
        let mut items = items;
        if let Some(completed) = items.pop() {
            println!(
                "{}{}",
                if command.quiet { "" } else { "Completed: " },
                completed.name
            );
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
            println!(
                "{}{}",
                if command.quiet { "" } else { "Deleted: " },
                deleted.name
            );
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
    // TODO: Think on this. This limits practical size, but needs a change to the
    // save/load format and/or algorithms to scale.
    if let Ok(items) = data::load(command) {
        if !items.is_empty() {
            let mut items = items;
            items.reverse();
            if command.quiet {
                items.iter().for_each(|item| println!("{}", item.name))
            } else {
                println!("Curr: {}", items[0].name);
                items
                    .iter()
                    .enumerate()
                    .skip(1)
                    .for_each(|(n, item)| println!("{: >4}: {}", n, item.name))
            }
        }
    }
}

fn list_all(command: &Command) {
    // TODO: In a stacks-of-stacks world, this should do more.
    list(command)
}

fn is_empty(command: &Command) {
    if let Ok(items) = data::load(command) {
        let is_empty = items.is_empty();
        if !command.quiet {
            println!("{}", is_empty);
        }
        if !is_empty {
            // TODO: This would be better as an Err, once everything returns Result
            panic!()
        }
    }
}

fn length(command: &Command) {
    if let Ok(items) = data::load(command) {
        println!(
            "{}{}",
            if command.quiet { "" } else { "Items: " },
            items.len()
        )
    }
}

fn peek(command: &Command) {
    if let Ok(items) = data::load(command) {
        if !items.is_empty() {
            println!(
                "{}{}",
                if command.quiet { "" } else { "Curr: " },
                items.last().unwrap().name
            )
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
