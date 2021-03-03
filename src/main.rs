use chrono::{NaiveDate, NaiveDateTime};
use clap::{App, Arg, SubCommand};
use serde::{Serialize, Deserialize};
use std::{env, fs, ops::Add};

#[derive(Serialize, Deserialize, Debug)]
struct Rank {
    name: String,
    created: NaiveDateTime,
    done: Option<NaiveDateTime>
}

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

    if let Ok(contents) = fs::read_to_string(env::var("HOME").unwrap().add("/.local/share/rank/test.json")) {
        println!("Contents: {:?}", serde_json::from_str::<Rank>(&contents));
    }
}
