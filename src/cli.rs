use crate::actions::{Action, ActionInput, ActionMetadata, Command};
use crate::output::{NoiseLevel, OutputFormat};
use crate::data::Item;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::iter;
use Action::*;

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version of the CLI.
pub const SIGI_VERSION: &str = "1.1.0";

const DEFAULT_STACK_NAME: &str = "sigi";

fn get_app() -> App<'static, 'static> {
    let peek = Peek.data();

    App::new("sigi")
        .version(SIGI_VERSION)
        .about("An organizational tool.")
        .arg(
            Arg::with_name("stack")
                .short("t")
                .long("stack")
                .value_name("STACK")
                .visible_aliases(&["topic", "about", "namespace"])
                .help("Manage items in a specific stack")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Omit any leading labels or symbols. Recommended for use in shell scripts"),
        )
        .arg(
            Arg::with_name("silent")
                .short("s")
                .long("silent")
                .help("Omit any output at all."),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .visible_alias("noisy")
                .help("Print more information, like when an item was created."),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .help("Use a programmatic format. Options include [csv, json, tsv]. Not compatible with quiet/silent/verbose.")
        )
        .subcommand(
            SubCommand::with_name(peek.name)
                .about("Show the first item. (This is the default behavior when no command is given)")
                .visible_aliases(&peek.aliases)
        )
        .subcommands(vec![
            Create(Item::new("")),
            Complete,
            Delete,
            DeleteAll,
            List,
            Head(None),
            Tail(None),
            Pick(vec![]),
            Length,
            Move(String::new()),
            MoveAll(String::new()),
            IsEmpty,
            Next,
            Swap,
            Rot,
        ]
        .into_iter()
        .map(subcommand_for))
}

/// Parses command line arguments and returns a single `sigi::actions::Command`.
pub fn parse_command() -> Command {
    let create = Create(Item::new(""));
    let head = Head(None);
    let tail = Tail(None);
    let pick = Pick(vec![]);
    let move_item = Move(String::new());
    let move_all = MoveAll(String::new());

    let matches = get_app().get_matches();

    let maybe_command = |action: &Action| matches.subcommand_matches(action.data().name);

    let action: Action = if let Some(matches) = maybe_command(&create) {
        if let Some(name_bits) = matches.values_of(arg_name_for(&create.data())) {
            let name = name_bits.collect::<Vec<_>>().join(" ");
            Create(Item::new(&name))
        } else {
            error_no_command(create.data().name, matches.is_present("silent"))
        }
    } else if let Some(matches) = maybe_command(&pick) {
        let indices = matches
            .values_of(arg_name_for(&pick.data()))
            .unwrap()
            .map(|i| usize::from_str_radix(&i, 10).unwrap())
            .collect();
        Pick(indices)
    } else if let Some(n) = maybe_command(&head) {
        let n = n
            .value_of(arg_name_for(&head.data()))
            .map(only_digits)
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Head(n)
    } else if let Some(n) = maybe_command(&tail) {
        let n = n
            .value_of(arg_name_for(&tail.data()))
            .map(only_digits)
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Tail(n)
    } else if let Some(dest) = maybe_command(&move_item) {
        let dest = dest
            .value_of(arg_name_for(&move_item.data()))
            .unwrap()
            .to_string();
        Move(dest)
    } else if let Some(dest) = maybe_command(&move_all) {
        let dest = dest
            .value_of(arg_name_for(&move_all.data()))
            .unwrap()
            .to_string();
        MoveAll(dest)
    } else if let Some(command) = vec![
        Complete, Delete, DeleteAll, List, Length, IsEmpty, Next, Swap, Rot,
    ]
    .into_iter()
    .find(|action| maybe_command(&action).is_some())
    {
        command
    } else {
        Peek
    };

    let stack = matches
        .value_of("stack")
        .unwrap_or(DEFAULT_STACK_NAME)
        .to_owned();

    let format = get_format(matches);

    Command {
        action,
        stack,
        format,
    }
}

fn get_format(matches: ArgMatches) -> OutputFormat {
    let default_format = OutputFormat::Human(NoiseLevel::Normal);

    if matches.is_present("verbose") {
        OutputFormat::Human(NoiseLevel::Verbose)
    } else if matches.is_present("silent") {
        OutputFormat::Silent
    } else if matches.is_present("quiet") {
        OutputFormat::Human(NoiseLevel::Quiet)
    } else if let Some(fmt) = matches.value_of("format") {
        match fmt {
            "csv" => OutputFormat::Csv,
            "json" => OutputFormat::Json,
            "tsv" => OutputFormat::Tsv,
            _ => default_format,
        }
    } else {
        default_format
    }
}

fn only_digits(s: &str) -> String {
    iter::once('0')
        .chain(s.chars())
        .filter(|c| c.is_digit(10))
        .collect::<String>()
}

fn arg_name_for<'a>(data: &ActionMetadata<'a>) -> &'a str {
    match data.input.as_ref().unwrap() {
        ActionInput::OptionalSingle(arg) => arg,
        ActionInput::RequiredSingle(arg) => arg,
        ActionInput::RequiredSlurpy(arg) => arg,
    }
}

fn subcommand_for<'a, 'b>(action: Action) -> App<'a, 'b> {
    let data = action.data();

    let cmd = SubCommand::with_name(data.name)
        .about(data.description)
        .visible_aliases(&data.aliases);

    if data.input.is_none() {
        return cmd;
    }

    let (is_required, is_multiple) = match data.input.clone().unwrap() {
        ActionInput::OptionalSingle(_) => (false, false),
        ActionInput::RequiredSingle(_) => (true, false),
        ActionInput::RequiredSlurpy(_) => (true, true),
    };

    cmd.arg(
        Arg::with_name(arg_name_for(&data))
            .takes_value(true)
            .required(is_required)
            .multiple(is_multiple),
    )
}

fn error_no_command(name: &str, silent: bool) -> Action {
    if !silent {
        eprintln!("Error, not enough arguments given for: {}", name);
    }
    Peek
}
