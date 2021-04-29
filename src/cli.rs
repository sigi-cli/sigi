use crate::actions::{Action, ActionInput, ActionMetadata, Command, NoiseLevel};
use crate::data::Item;
use clap::{App, Arg, SubCommand};
use Action::*;

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version (0.2.5) of the CLI.
pub const SIGI_VERSION: &str = "0.2.5";

const DEFAULT_STACK_NAME: &str = "sigi";

/// Parses command line arguments and returns a single `sigi::actions::Command`.
pub fn get_action() -> Command {
    let peek = Peek.data();
    let create = Create(Item::new(""));
    let head = Head(None);
    let tail = Tail(None);
    let pick = Pick(vec![]);
    let move_item = Move(String::new());
    let move_all = MoveAll(String::new());

    let matches = App::new("sigi")
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
                .help("Enable verbose output."),
        )
        .subcommands(
            vec![
                vec![SubCommand::with_name(peek.name)
                    .about(&*format!(
                        "{} {}",
                        peek.description, "(This is the default behavior when no command is given)"
                    ))
                    .visible_aliases(&peek.aliases)],
                vec![
                    create.clone(),
                    Complete,
                    Delete,
                    DeleteAll,
                    List,
                    head.clone(),
                    tail.clone(),
                    pick.clone(),
                    Length,
                    move_item.clone(),
                    move_all.clone(),
                    IsEmpty,
                    Next,
                    Swap,
                    Rot,
                ]
                .into_iter()
                .map(subcommand_for)
                .collect(),
            ]
            .concat(),
        )
        .get_matches();

    let is_command = |action: &Action| matches.subcommand_matches(action.data().name);

    let action: Action = if let Some(matches) = is_command(&create) {
        if let Some(name_bits) = matches.values_of(arg_name_for(&create.data())) {
            let name = name_bits.collect::<Vec<_>>().join(" ");
            Create(Item::new(&name))
        } else {
            error_no_command(create.data().name, matches.is_present("silent"))
        }
    } else if let Some(matches) = is_command(&pick) {
        let indices = matches
            .values_of(arg_name_for(&pick.data()))
            .unwrap()
            .map(|i| usize::from_str_radix(&i, 10).unwrap())
            .collect();
        Pick(indices)
    } else if let Some(n) = is_command(&head) {
        let n = n
            .value_of(arg_name_for(&head.data()))
            // FIXME: Validate it's numeric or fail gracefully
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Head(n)
    } else if let Some(n) = is_command(&tail) {
        let n = n
            .value_of(arg_name_for(&tail.data()))
            // FIXME: Validate it's numeric or fail gracefully
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Tail(n)
    } else if let Some(dest) = is_command(&move_item) {
        let dest = dest
            .value_of(arg_name_for(&move_item.data()))
            .unwrap()
            .to_string();
        Move(dest)
    } else if let Some(dest) = is_command(&move_all) {
        let dest = dest
            .value_of(arg_name_for(&move_all.data()))
            .unwrap()
            .to_string();
        MoveAll(dest)
    } else if let Some(command) = vec![
        Complete, Delete, DeleteAll, List, Length, IsEmpty, Next, Swap, Rot,
    ]
    .iter()
    .find(|action| is_command(&action).is_some())
    {
        command.clone()
    } else {
        Peek
    };

    let stack = matches
        .value_of("stack")
        .unwrap_or(DEFAULT_STACK_NAME)
        .to_owned();

    let noise = if matches.is_present("verbose") {
        NoiseLevel::Verbose
    } else if matches.is_present("silent") {
        NoiseLevel::Silent
    } else if matches.is_present("quiet") {
        NoiseLevel::Quiet
    } else {
        NoiseLevel::Normal
    };

    Command {
        action,
        stack,
        noise,
    }
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
    match data.input {
        None => cmd,
        Some(input) => match input {
            ActionInput::OptionalSingle(arg) => cmd.arg(Arg::with_name(arg).takes_value(true)),
            ActionInput::RequiredSingle(arg) => {
                cmd.arg(Arg::with_name(arg).takes_value(true).required(true))
            }
            ActionInput::RequiredSlurpy(arg) => cmd.arg(
                Arg::with_name(arg)
                    .takes_value(true)
                    .required(true)
                    .multiple(true),
            ),
        },
    }
}

fn error_no_command(name: &str, silent: bool) -> Action {
    if !silent {
        eprintln!("Error, not enough arguments given for: {}", name);
    }
    Peek
}
