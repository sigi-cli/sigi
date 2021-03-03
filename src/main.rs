use chrono::{DateTime, Local};
use clap::{App, Arg, SubCommand};
use serde::{Serialize, Deserialize};
use std::{env, fs, iter};

#[derive(Serialize, Deserialize, Debug)]
struct Rank {
    name: String,
    created: DateTime<Local>,
    done: Option<DateTime<Local>>
}

const RANK_DATA_PATH: [&str; 4] = [".local", "share", "rank", "test.json"];

fn main() {
    let matches = App::new("rank")
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
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
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
        .get_matches();

    println!("Matches: {:?}", matches);

    let data_path: String = iter::once(env::var("HOME").unwrap())
        .chain(RANK_DATA_PATH.iter().map(|s| s.to_string()))
        .collect::<Vec<_>>()
        .join(&std::path::MAIN_SEPARATOR.to_string());

    let rank: Rank = Rank { name: String::from("John Cena"), created: Local::now(), done: None } ;

    if let Ok(Ok(something)) = serde_json::to_string(&rank).map(|s| fs::write(data_path.clone(), s)) {
        println!("Wrote something: {:?}", something);
    }

    if let Ok(contents) = fs::read_to_string(data_path) {
        println!("Contents: {:?}", serde_json::from_str::<Rank>(&contents));
    }
}
