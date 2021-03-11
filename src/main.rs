use chrono::{DateTime, Local};
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{env, fs, iter};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    created: DateTime<Local>,
    succeeded: Option<DateTime<Local>>,
    failed: Option<DateTime<Local>>,
}

type Items = Vec<Item>;

const RANK_DATA_PATH: [&str; 3] = [".local", "share", "sigi"];

const CREATE_ALIASES: [&str; 3] = ["do", "start", "new"];
const COMPLETE_ALIASES: [&str; 3] = ["done", "finish", "fulfill"];
const DELETE_ALIASES: [&str; 3] = ["drop", "abandon", "retire"];
const LIST_ALIASES: [&str; 0] = [];
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
                .arg(Arg::with_name("item").value_name("ITEM").required(true)),
            SubCommand::with_name("complete")
                .about("Marks the current item as successfully completed")
                .aliases(&COMPLETE_ALIASES),
            SubCommand::with_name("delete")
                .about("Marks the current item as unsuccessfully completed")
                .aliases(&DELETE_ALIASES),
            SubCommand::with_name("list")
                .about("Lists the current priority items")
                .aliases(&LIST_ALIASES),
            SubCommand::with_name("all")
                .about("Lists all items")
                .aliases(&ALL_ALIASES),
        ])
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        if let Some(item) = matches.value_of("item") {
            println!("Creating: {:?}", item);
            let items: Items = vec![Item {
                name: String::from(item),
                created: Local::now(),
                succeeded: None,
                failed: None,
            }];
            sigi_save(items).unwrap();
            return;
        }
    }

    if matches.subcommand_matches("complete").is_some() {
        println!("Good done.");
        return;
    }

    if matches.subcommand_matches("delete").is_some() {
        println!("Bad done.");
        return;
    }

    if matches.subcommand_matches("list").is_some() {
        println!("Listing.");
        return;
    }

    if matches.subcommand_matches("all").is_some() {
        println!("ALL!");
        return;
    }

    println!("Matches: {:?}", matches);

    let data_path: String = sigi_data_path("test.json");

    let items: Items = vec![Item {
        name: String::from("John Cena"),
        created: Local::now(),
        succeeded: None,
        failed: None,
    }];

    if let Ok(Ok(something)) =
        serde_json::to_string(&items).map(|s| fs::write(data_path.clone(), s))
    {
        println!("Wrote something: {:?}", something);
    }

    if let Ok(contents) = fs::read_to_string(data_path) {
        println!("Contents: {:?}", serde_json::from_str::<Items>(&contents));
    }
}

fn sigi_save(items: Items) -> Result<(), impl Error> {
    let data_path: String = sigi_data_path("sigi.json");
    let json: String = serde_json::to_string(&items)?;
    fs::write(data_path.clone(), json)
}

fn sigi_data_path(filename: impl Into<String>) -> String {
    iter::once(env::var("HOME").or(env::var("HOMEPATH")).unwrap())
        .chain(RANK_DATA_PATH.iter().map(|s| s.to_string()))
        .chain(iter::once(filename.into()))
        .collect::<Vec<_>>()
        .join(&std::path::MAIN_SEPARATOR.to_string())
}
