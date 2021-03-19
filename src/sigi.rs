/// Sigi, a tool for organizing.
///
/// TODO: Add guidance on using sigi as a library.
///
use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};

mod data;
mod items;

use items::Item;

struct Mode<'a> {
    topic: &'a str,
    quiet: bool,
}

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
pub const SIGI_VERSION: &str = "0.1.6";

const CREATE_ALIASES: [&str; 5] = ["push", "add", "do", "start", "new"];
const COMPLETE_ALIASES: [&str; 3] = ["done", "finish", "fulfill"];
const DELETE_ALIASES: [&str; 6] = ["pop", "remove", "cancel", "drop", "abandon", "retire"];
const LIST_ALIASES: [&str; 1] = ["show"];
const ALL_ALIASES: [&str; 0] = [];
const NEXT_ALIASES: [&str; 3] = ["later", "punt", "bury"];
const SWAP_ALIASES: [&str; 0] = [];
const ROT_ALIASES: [&str; 1] = ["rotate"];

pub fn run() {
    // TODO: Make some middle layer between clap ideas and the core logic
    let matches: ArgMatches = App::new("sigi")
        .version(SIGI_VERSION)
        .about("An organizational tool.")
        .arg(
            Arg::with_name("topic")
                .short("t")
                .long("topic")
                .value_name("TOPIC")
                .help("Manage items in a specific topic")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Omit any leading labels or symbols. Recommended for use in shell scripts")
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
            SubCommand::with_name("next")
                .about("Moves the next item to current, and moves current to last.")
                .aliases(&NEXT_ALIASES),
            SubCommand::with_name("swap")
                .about("Swaps the two most current items.")
                .aliases(&SWAP_ALIASES),
            SubCommand::with_name("rot")
                .about("Rotates the three most current items.")
                .aliases(&ROT_ALIASES),
            SubCommand::with_name("peek")
                .about("Peek at the current item. (This is the default behavior if no command is given)")
                .aliases(&NEXT_ALIASES),
            // TODO: Need an idea of "organize" or "re-order"
            // TODO: Forthisms for near-top actions like swap/rot would be awesome
            // TODO: Need support for stack-of-stack management
        ])
        .get_matches();

    let mode = Mode {
        topic: matches.value_of("topic").unwrap_or("sigi"),
        quiet: matches.is_present("quiet"),
    };

    let command = |name: &str| matches.subcommand_matches(name);
    let command_is = |name: &str| command(name).is_some();

    if let Some(matches) = command("create") {
        create(&mode, matches)
    } else if command_is("complete") {
        complete(&mode)
    } else if command_is("delete") {
        delete(&mode)
    } else if command_is("list") {
        list(&mode)
    } else if command_is("all") {
        all(&mode)
    } else if command_is("next") {
        next(&mode)
    } else if command_is("swap") {
        swap(&mode)
    } else if command_is("rot") {
        rot(&mode)
    } else {
        peek(&mode)
    }
}

// TODO: Refactor. The repetition in function signatures suggests struct { &str, Option<ArgMatches> }
// TODO: Return Result<(), Error> - some error cases are not covered (e.g. create with no content)

fn create(mode: &Mode, matches: &ArgMatches) {
    if let Some(name_bits) = matches.values_of("name") {
        let name = name_bits.collect::<Vec<_>>().join(" ");
        println!("{}{}", if mode.quiet { "" } else { "Creating: " }, name);
        let item = Item {
            name,
            created: Local::now(),
            succeeded: None,
            failed: None,
        };
        if let Ok(items) = data::load(mode.topic) {
            let mut items = items;
            items.push(item);
            data::save(mode.topic, items).unwrap();
        } else {
            data::save(mode.topic, vec![item]).unwrap();
        }
    }
}

fn complete(mode: &Mode) {
    if let Ok(items) = data::load(mode.topic) {
        let mut items = items;
        if let Some(completed) = items.pop() {
            println!(
                "{}{}",
                if mode.quiet { "" } else { "Completed: " },
                completed.name
            );
            // TODO: Archive instead of delete. (update, save somewhere recoverable)
            // TODO: Might be nice to have a "history" command for viewing these.
        }
        data::save(mode.topic, items).unwrap();
    }
}

fn delete(mode: &Mode) {
    if let Ok(items) = data::load(mode.topic) {
        let mut items = items;
        if let Some(deleted) = items.pop() {
            println!(
                "{}{}",
                if mode.quiet { "" } else { "Deleted: " },
                deleted.name
            );
            // TODO: Archive instead of delete? (i.e. save somewhere recoverable)
            // Might allow an easy "undo" or "undelete"; would need a "purge" idea
            // TODO: Might be nice to have a "history" command for viewing these
        }
        data::save(mode.topic, items).unwrap();
    }
}

fn list(mode: &Mode) {
    // TODO: Think on this. This limits practical size, but needs a change to the
    // save/load format and/or algorithms to scale.
    if let Ok(items) = data::load(mode.topic) {
        if !items.is_empty() {
            let mut items = items;
            items.reverse();
            if mode.quiet {
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

fn all(mode: &Mode) {
    // TODO: In a stacks-of-stacks world, this should do more.
    list(mode)
}

fn peek(mode: &Mode) {
    if let Ok(items) = data::load(mode.topic) {
        if !items.is_empty() {
            println!(
                "{}{}",
                if mode.quiet { "" } else { "Curr: " },
                items.last().unwrap().name
            )
        }
    }
}

fn swap(mode: &Mode) {
    if let Ok(items) = data::load(mode.topic) {
        let mut items = items;
        if items.len() < 2 {
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        items.push(a);
        items.push(b);

        data::save(mode.topic, items).unwrap();
        peek(mode)
    }
}

fn rot(mode: &Mode) {
    if let Ok(items) = data::load(mode.topic) {
        let mut items = items;
        if items.len() < 3 {
            swap(mode);
            return;
        }
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        let c = items.pop().unwrap();
        items.push(a);
        items.push(c);
        items.push(b);

        data::save(mode.topic, items).unwrap();
        peek(mode)
    }
}

fn next(mode: &Mode) {
    if let Ok(items) = data::load(mode.topic) {
        let mut items = items;
        if items.is_empty() {
            return;
        }
        let to_the_back = items.pop().unwrap();
        items.insert(0, to_the_back);

        data::save(mode.topic, items).unwrap();
        peek(mode)
    }
}
