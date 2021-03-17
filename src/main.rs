use chrono::{DateTime, Local};
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    created: DateTime<Local>,
    succeeded: Option<DateTime<Local>>,
    failed: Option<DateTime<Local>>,
}

type Items = Vec<Item>;

// TODO: For non-unixy systems, need to use std::path::MAIN_SEPARATOR
const SIGI_DATA_PATH: &str = ".local/share/sigi";

const CREATE_ALIASES: [&str; 4] = ["add", "do", "start", "new"];
const COMPLETE_ALIASES: [&str; 3] = ["done", "finish", "fulfill"];
const DELETE_ALIASES: [&str; 5] = ["remove", "cancel", "drop", "abandon", "retire"];
const LIST_ALIASES: [&str; 1] = ["show"];
const ALL_ALIASES: [&str; 0] = [];

fn main() {
    // TODO: Make some middle layer between clap ideas and the core logic
    let matches: ArgMatches = App::new("sigi")
        .version("1.0")
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
            if let Ok(items) = sigi_load() {
                let mut items = items;
                items.push(item);
                sigi_save(items).unwrap();
            } else {
                sigi_save(vec![item]).unwrap();
            }
            return;
        }
    }

    // Complete
    if matches.subcommand_matches("complete").is_some() {
        if let Ok(items) = sigi_load() {
            let mut items = items;
            if let Some(completed) = items.pop() {
                println!("Completed: {}", completed.name);
                // TODO: Archive instead of delete. (update, save somewhere recoverable)
                // TODO: Might be nice to have a "history" command for viewing these.
            }
            sigi_save(items).unwrap();
        }
        return;
    }

    // Delete
    if matches.subcommand_matches("delete").is_some() {
        if let Ok(items) = sigi_load() {
            let mut items = items;
            if let Some(deleted) = items.pop() {
                println!("Deleted: {}", deleted.name);
                // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
                // Might allow an easy "undo" or "undelete"; would need a "purge" idea
                // TODO: Might be nice to have a "history" command for viewing these
            }
            sigi_save(items).unwrap();
        }
        return;
    }

    // List
    if matches.subcommand_matches("list").is_some() {
        if let Ok(items) = sigi_load() {
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
        if let Ok(items) = sigi_load() {
            if !items.is_empty() {
                let mut items = items;
                items.reverse();
                items.iter().for_each(|item| println!("- {}", item.name));
            }
        }
        return;
    }

    // No args
    if let Ok(items) = sigi_load() {
        if !items.is_empty() {
            println!("{}", items.last().unwrap().name)
        }
    }
}

// TODO: Move data management to its own lib file
// TODO: Allow an idea of "stack of stacks"
// TODO: Allow namespaces
// TODO: Figure out a good naming algorithm (maybe numeric?)

fn sigi_save(items: Items) -> Result<(), impl Error> {
    let data_path: String = sigi_file("sigi.json");
    let json: String = serde_json::to_string(&items).unwrap();
    let result = fs::write(&data_path, &json);
    if result.is_err() && result.as_ref().unwrap_err().kind() == std::io::ErrorKind::NotFound {
        fs::create_dir_all(sigi_path()).unwrap();
        fs::write(data_path, json)
    } else {
        result
    }
}

fn sigi_load() -> Result<Items, impl Error> {
    let data_path: String = sigi_file("sigi.json");
    let read_result = fs::read_to_string(data_path);
    if read_result.is_err()
        && read_result.as_ref().unwrap_err().kind() == std::io::ErrorKind::NotFound
    {
        Ok(vec![])
    } else {
        let json = read_result.unwrap();
        serde_json::from_str(&json)
    }
}

fn sigi_path() -> String {
    format!("{}/{}", env::var("HOME").unwrap(), SIGI_DATA_PATH)
}

fn sigi_file(filename: &str) -> String {
    format!("{}/{}", sigi_path(), filename)
}
