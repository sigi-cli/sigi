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
        println!("Good done.");
        return;
    }

    // Delete
    if matches.subcommand_matches("delete").is_some() {
        if let Ok(items) = sigi_load() {
            let mut items = items;
            if let Some(maybe_deleted) = items.pop() {
                println!("Deleted: {}", maybe_deleted.name);
                // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
                // Might allow an easy "undo" or "undelete"; would need a "purge" idea
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
        println!("ALL!");
        return;
    }

    // No args
    if let Ok(items) = sigi_load() {
        if !items.is_empty() {
            println!("{}", items.last().unwrap().name)
        }
    }
}

fn sigi_save(items: Items) -> Result<(), impl Error> {
    let data_path: String = sigi_data_file("sigi.json");
    let json: String = serde_json::to_string(&items).unwrap();
    fs::write(data_path, json)
}

fn sigi_load() -> Result<Items, impl Error> {
    let data_path: String = sigi_data_file("sigi.json");
    let json: String = fs::read_to_string(data_path).unwrap();
    serde_json::from_str(&json)
}

fn sigi_data_file(filename: &str) -> String {
    // TODO: Create data directory if it doesn't exist.
    format!(
        "{}/{}/{}",
        env::var("HOME").unwrap(),
        SIGI_DATA_PATH,
        filename
    )
}
