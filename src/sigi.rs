use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};

mod data;
mod items;

use items::Item;

const CREATE_ALIASES: [&str; 4] = ["add", "do", "start", "new"];
const COMPLETE_ALIASES: [&str; 3] = ["done", "finish", "fulfill"];
const DELETE_ALIASES: [&str; 5] = ["remove", "cancel", "drop", "abandon", "retire"];
const LIST_ALIASES: [&str; 1] = ["show"];
const ALL_ALIASES: [&str; 0] = [];

pub fn run() {
    // TODO: Make some middle layer between clap ideas and the core logic
    let matches: ArgMatches = App::new("sigi")
        // TODO: Get the version from Cargo.toml? (If possible, at compile time)
        .version("0.1.3")
        .about("An organizational tool.")
        .arg(
            Arg::with_name("topic")
                .short("t")
                .long("topic")
                .value_name("TOPIC")
                .help("Manage items in a specific topic")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("create")
                .about("Creates a new item")
                .aliases(&CREATE_ALIASES)
                .arg(
                    Arg::with_name("name")
                        .value_name("NAME")
                        .required(true)
                        .multiple(true),
                ),
            SubCommand::with_name("complete")
                .about("Marks the current item as successfully completed")
                .aliases(&COMPLETE_ALIASES),
            SubCommand::with_name("delete")
                .about("Removes the current item")
                .aliases(&DELETE_ALIASES),
            SubCommand::with_name("list")
                .about("Lists the current priority items")
                .aliases(&LIST_ALIASES),
            SubCommand::with_name("all")
                .about("Lists all items")
                .aliases(&ALL_ALIASES),
            // TODO: Need an idea of "organize" or "re-order"
            // TODO: Forthisms for near-top actions like swap/rot would be awesome
            // TODO: Need an idea of "later" or "back of the line"
            // TODO: Need support for stack-of-stack management
        ])
        .get_matches();

    let topic = matches.value_of("topic").unwrap_or("sigi");

    // Create
    if let Some(matches) = matches.subcommand_matches("create") {
        if let Some(name_bits) = matches.values_of("name") {
            let name = name_bits.collect::<Vec<_>>().join(" ");
            println!("Creating: {}", name);
            let item = Item {
                name,
                created: Local::now(),
                succeeded: None,
                failed: None,
            };
            if let Ok(items) = data::load(topic) {
                let mut items = items;
                items.push(item);
                data::save(topic, items).unwrap();
            } else {
                data::save(topic, vec![item]).unwrap();
            }
            return;
        }
    }

    // Complete
    if matches.subcommand_matches("complete").is_some() {
        if let Ok(items) = data::load(topic) {
            let mut items = items;
            if let Some(completed) = items.pop() {
                println!("Completed: {}", completed.name);
                // TODO: Archive instead of delete. (update, save somewhere recoverable)
                // TODO: Might be nice to have a "history" command for viewing these.
            }
            data::save(topic, items).unwrap();
        }
        return;
    }

    // Delete
    if matches.subcommand_matches("delete").is_some() {
        if let Ok(items) = data::load(topic) {
            let mut items = items;
            if let Some(deleted) = items.pop() {
                println!("Deleted: {}", deleted.name);
                // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
                // Might allow an easy "undo" or "undelete"; would need a "purge" idea
                // TODO: Might be nice to have a "history" command for viewing these
            }
            data::save(topic, items).unwrap();
        }
        return;
    }

    // List
    if matches.subcommand_matches("list").is_some() {
        if let Ok(items) = data::load(topic) {
            if !items.is_empty() {
                let mut items = items;
                items.reverse();
                items.iter().for_each(|item| println!("- {}", item.name));
            }
        }
        return;
    }

    // All
    if matches.subcommand_matches("all").is_some() {
        if let Ok(items) = data::load(topic) {
            if !items.is_empty() {
                let mut items = items;
                items.reverse();
                items.iter().for_each(|item| println!("- {}", item.name));
            }
        }
        return;
    }

    // No args
    if let Ok(items) = data::load(topic) {
        if !items.is_empty() {
            println!("{}", items.last().unwrap().name)
        }
    }
}
