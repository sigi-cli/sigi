use crate::sigi::data;
use crate::sigi::items::Item;
use chrono::Local;

pub enum Command {
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

pub struct Action {
    pub command: Command,
    pub topic: String,
    pub quiet: bool,
}

impl Action {
    pub fn act(&self) {
        match self.command {
            Command::Create(_) => create(self),
            Command::Complete => complete(self),
            Command::Delete => delete(self),
            Command::DeleteAll => delete_all(self),
            Command::List => list(self),
            Command::ListAll => list_all(self),
            Command::IsEmpty => is_empty(self),
            Command::Length => length(self),
            Command::Next => next(self),
            Command::Swap => swap(self),
            Command::Rot => rot(self),
            Command::Peek => peek(self),
        }
    }
}
// TODO: Refactor. The repetition in function signatures suggests struct { &str, Option<ArgMatches> }
// TODO: Return Result<(), Error> - some error cases are not covered (e.g. create with no content)

fn create(action: &Action) {
    if let Command::Create(name) = &action.command {
        println!("{}{}", if action.quiet { "" } else { "Creating: " }, name);
        let item = Item {
            name: name.to_string(),
            created: Local::now(),
            succeeded: None,
            failed: None,
        };
        if let Ok(items) = data::load(action) {
            let mut items = items;
            items.push(item);
            data::save(action, items).unwrap();
        } else {
            data::save(action, vec![item]).unwrap();
        }
    }
}

fn complete(action: &Action) {
    if let Ok(items) = data::load(action) {
        let mut items = items;
        if let Some(completed) = items.pop() {
            println!(
                "{}{}",
                if action.quiet { "" } else { "Completed: " },
                completed.name
            );
            // TODO: Archive instead of delete. (update, save somewhere recoverable)
            // TODO: Might be nice to have a "history" command for viewing these.
        }
        data::save(action, items).unwrap();
    }
}

fn delete(action: &Action) {
    if let Ok(items) = data::load(action) {
        let mut items = items;
        if let Some(deleted) = items.pop() {
            println!(
                "{}{}",
                if action.quiet { "" } else { "Deleted: " },
                deleted.name
            );
            // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
            // Might allow an easy "undo" or "undelete"; would need a "purge" idea
            // TODO: Might be nice to have a "history" command for viewing these
        }
        data::save(action, items).unwrap();
    }
}

fn delete_all(action: &Action) {
    // TODO: In a stacks-of-stacks world, this will need to do more.
    delete(action)
}

fn list(action: &Action) {
    // TODO: Think on this. This limits practical size, but needs a change to the
    // save/load format and/or algorithms to scale.
    if let Ok(items) = data::load(action) {
        if !items.is_empty() {
            let mut items = items;
            items.reverse();
            if action.quiet {
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

fn list_all(action: &Action) {
    // TODO: In a stacks-of-stacks world, this should do more.
    list(action)
}

fn is_empty(action: &Action) {
    if let Ok(items) = data::load(action) {
        let is_empty = items.is_empty();
        if !action.quiet {
            println!("{}", is_empty);
        }
        if !is_empty {
            // TODO: This would be better as an Err, once everything returns Result
            panic!()
        }
    }
}

fn length(action: &Action) {
    if let Ok(items) = data::load(action) {
        println!(
            "{}{}",
            if action.quiet { "" } else { "Items: " },
            items.len()
        )
    }
}

fn peek(action: &Action) {
    if let Ok(items) = data::load(action) {
        if !items.is_empty() {
            println!(
                "{}{}",
                if action.quiet { "" } else { "Curr: " },
                items.last().unwrap().name
            )
        }
    }
}

fn swap(action: &Action) {
    if let Ok(items) = data::load(action) {
        let mut items = items;
        if items.len() < 2 {
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        items.push(a);
        items.push(b);

        data::save(action, items).unwrap();
        peek(action)
    }
}

fn rot(action: &Action) {
    if let Ok(items) = data::load(action) {
        let mut items = items;
        if items.len() < 3 {
            swap(action);
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        let c = items.pop().unwrap();
        items.push(a);
        items.push(c);
        items.push(b);

        data::save(action, items).unwrap();
        peek(action)
    }
}

fn next(action: &Action) {
    if let Ok(items) = data::load(action) {
        let mut items = items;
        if items.is_empty() {
            return;
        }
        let to_the_back = items.pop().unwrap();
        items.insert(0, to_the_back);

        data::save(action, items).unwrap();
        peek(action)
    }
}
