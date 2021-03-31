use crate::actions::{Action, ActionInput, Command, NoiseLevel};
use crate::data::Item;
use clap::{App, Arg, SubCommand};
use Action::*;

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version (0.2.1) of the CLI.
pub const SIGI_VERSION: &str = "0.2.1";

const DEFAULT_STACK_NAME: &str = "sigi";

/// Parses command line arguments and returns a single `sigi::actions::Command`.
pub fn get_action() -> Command {
    let peek = Peek.data();
    let create = Create(Item::new("")).data();
    let ActionInput::RequiredSlurpy(create_arg) = create.input.unwrap();
    let complete = Complete.data();
    let delete = Delete.data();
    let delete_all = DeleteAll.data();
    let list = List.data();
    let length = Length.data();
    let is_empty = IsEmpty.data();
    let next = Next.data();
    let swap = Swap.data();
    let rot = Rot.data();

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
        // TODO: Collapse repetition
        .subcommands(vec![
            SubCommand::with_name(peek.name)
                .about(&*format!(
                    "{} {}",
                    peek.description, "(This is the default behavior when no command is given)"
                ))
                .visible_aliases(&peek.aliases),
            SubCommand::with_name(create.name)
                .about(create.description)
                .visible_aliases(&create.aliases)
                .arg(
                    Arg::with_name(create_arg)
                        .value_name(&create_arg.to_uppercase())
                        .required(true)
                        .multiple(true),
                ),
            SubCommand::with_name(complete.name)
                .about(complete.description)
                .visible_aliases(&complete.aliases),
            SubCommand::with_name(delete.name)
                .about(delete.description)
                .visible_aliases(&delete.aliases),
            SubCommand::with_name(delete_all.name)
                .about(delete_all.description)
                .visible_aliases(&delete_all.aliases),
            SubCommand::with_name(list.name)
                .about(list.description)
                .visible_aliases(&list.aliases),
            SubCommand::with_name(length.name)
                .about(length.description)
                .visible_aliases(&length.aliases),
            SubCommand::with_name(is_empty.name)
                .about(is_empty.description)
                .visible_aliases(&is_empty.aliases),
            SubCommand::with_name(next.name)
                .about(next.description)
                .visible_aliases(&next.aliases),
            SubCommand::with_name(swap.name)
                .about(swap.description)
                .visible_aliases(&swap.aliases),
            SubCommand::with_name(rot.name)
                .about(rot.description)
                .visible_aliases(&rot.aliases),
            // TODO: Need an idea of "organize" or "re-order"
            // TODO: Need support for stack-of-stack management
        ])
        .get_matches();

    let command_is_opt = |name: &str| matches.subcommand_matches(name);
    let command_is = |name: &str| command_is_opt(name).is_some();

    let silent = matches.is_present("silent");

    let action: Action = if let Some(matches) = command_is_opt(create.name) {
        if let Some(name_bits) = matches.values_of(create_arg) {
            let name = name_bits.collect::<Vec<_>>().join(" ");
            Create(Item::new(&name))
        } else {
            error_no_command(create.name, silent)
        }
    } else if command_is(complete.name) {
        Complete
    } else if command_is(delete.name) {
        Delete
    } else if command_is(delete_all.name) {
        DeleteAll
    } else if command_is(list.name) {
        List
    } else if command_is(length.name) {
        Length
    } else if command_is(is_empty.name) {
        IsEmpty
    } else if command_is(next.name) {
        Next
    } else if command_is(swap.name) {
        Swap
    } else if command_is(rot.name) {
        Rot
    } else {
        Peek
    };

    let stack = matches
        .value_of("stack")
        .unwrap_or(DEFAULT_STACK_NAME)
        .to_owned();
    let quiet = matches.is_present("quiet");

    let noise = if silent {
        NoiseLevel::Silent
    } else if quiet {
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

fn error_no_command(name: &str, silent: bool) -> Action {
    if !silent {
        eprintln!("Error, not enough arguments given for: {}", name);
    }
    Peek
}
