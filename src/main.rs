use chrono::{DateTime, Local};
use clap::{App, Arg, SubCommand};
use serde::{Serialize, Deserialize};
use std::{env, fs, iter};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    created: DateTime<Local>,
    done: Option<DateTime<Local>>
}

type Tasks = Vec<Task>;

const RANK_DATA_PATH: [&str; 3] = [".local", "share", "sigi"];

fn main() {
    let matches = App::new("sigi")
        .version("1.0")
        .about("An organizational tool")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .get_matches();

    println!("Matches: {:?}", matches);

    let data_path: String = sigi_data_file("test.json");

    let tasks: Tasks = vec![Task { name: String::from("John Cena"), created: Local::now(), done: None }];

    if let Ok(Ok(something)) = serde_json::to_string(&tasks).map(|s| fs::write(data_path.clone(), s)) {
        println!("Wrote something: {:?}", something);
    }

    if let Ok(contents) = fs::read_to_string(data_path) {
        println!("Contents: {:?}", serde_json::from_str::<Tasks>(&contents));
    }
}

fn sigi_data_file(filename: impl Into<String>) -> String {
    iter::once(env::var("HOME").or(env::var("HOMEPATH")).unwrap())
        .chain(RANK_DATA_PATH.iter().map(|s| s.to_string()))
        .chain(iter::once(filename.into()))
        .collect::<Vec<_>>()
        .join(&std::path::MAIN_SEPARATOR.to_string())
}

