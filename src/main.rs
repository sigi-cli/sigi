use chrono::{DateTime, Local};
use clap::{App, Arg, SubCommand};
use serde::{Serialize, Deserialize};
use std::{env, fs, iter};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    created: DateTime<Local>,
    succeeded: Option<DateTime<Local>>,
    failed: Option<DateTime<Local>>
}

type Items = Vec<Item>;

const RANK_DATA_PATH: [&str; 3] = [".local", "share", "sigi"];
const CREATE_ALIASES: [&str; 3] = ["do", "start", "new"];
const COMPLETE_ALIASES: [&str; 3] = ["done", "finish", "fulfill"];
const DELETE_ALIASES: [&str; 3] = ["drop", "abandon", "retire"];

fn main() {
    let matches = App::new("sigi")
        .version("1.0")
        .about("An organizational tool")
        .args(vec![
            Arg::with_name("topic")
                .short("t")
                .long("topic")
                .value_name("TOPIC")
                .help("Manage items in a specific topic.")
                .takes_value(true),
        ])
        .subcommands(vec![
            SubCommand::with_name("create")
                .about("Creates a new item.")
                .aliases(&CREATE_ALIASES)
                .arg(Arg::with_name("ITEM").required(true).multiple(true)),
            SubCommand::with_name("complete")
                .about("Marks the current item as successfully completed.")
                .aliases(&COMPLETE_ALIASES),
            SubCommand::with_name("delete")
                .about("Marks the current item as unsuccessfully completed.")
                .aliases(&DELETE_ALIASES)
        ])
        .get_matches();

    println!("Matches: {:?}", matches);

    let data_path: String = sigi_data_file("test.json");

    let items: Items = vec![Item { name: String::from("John Cena"), created: Local::now(), succeeded: None, failed: None }];

    if let Ok(Ok(something)) = serde_json::to_string(&items).map(|s| fs::write(data_path.clone(), s)) {
        println!("Wrote something: {:?}", something);
    }

    if let Ok(contents) = fs::read_to_string(data_path) {
        println!("Contents: {:?}", serde_json::from_str::<Items>(&contents));
    }
}

fn sigi_data_file(filename: impl Into<String>) -> String {
    iter::once(env::var("HOME").or(env::var("HOMEPATH")).unwrap())
        .chain(RANK_DATA_PATH.iter().map(|s| s.to_string()))
        .chain(iter::once(filename.into()))
        .collect::<Vec<_>>()
        .join(&std::path::MAIN_SEPARATOR.to_string())
}
