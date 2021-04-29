use crate::actions::{Action, ActionInput, Command, NoiseLevel};
use crate::data::Item;
use clap::{App, Arg, SubCommand};
use Action::*;

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version (0.2.5) of the CLI.
pub const SIGI_VERSION: &str = "0.2.5";

const DEFAULT_STACK_NAME: &str = "sigi";

/// Parses command line arguments and returns a single `sigi::actions::Command`.
pub fn get_action() -> Command {
    // End the madness and refactor this!
    let peek = Peek.data();
    let create = Create(Item::new("")).data();
    let create_arg = match create.input.unwrap() {
        ActionInput::RequiredSlurpy(arg) => arg,
        _ => unreachable!(),
    };
    let head = Head(None).data();
    let head_arg = match head.input.unwrap() {
        ActionInput::OptionalSingle(arg) => arg,
        _ => unreachable!(),
    };
    let tail = Tail(None).data();
    let tail_arg = match tail.input.unwrap() {
        ActionInput::OptionalSingle(arg) => arg,
        _ => unreachable!(),
    };
    let pick = Pick(vec![]).data();
    let pick_arg = match pick.input.unwrap() {
        ActionInput::RequiredSlurpy(arg) => arg,
        _ => unreachable!(),
    };
    let move_item = Move(String::new()).data();
    let move_item_arg = match move_item.input.unwrap() {
        ActionInput::RequiredSingle(arg) => arg,
        _ => unreachable!(),
    };
    let move_all = MoveAll(String::new()).data();
    let move_all_arg = match move_all.input.unwrap() {
        ActionInput::RequiredSingle(arg) => arg,
        _ => unreachable!(),
    };

    let subcommand_for = |action: Action| {
        let data = action.data();
        SubCommand::with_name(data.name)
            .about(data.description)
            .visible_aliases(&data.aliases)
    };

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
            subcommand_for(Complete),
            subcommand_for(Delete),
            subcommand_for(DeleteAll),
            subcommand_for(List),
            SubCommand::with_name(head.name)
                .about(head.description)
                .visible_aliases(&head.aliases)
                .arg(Arg::with_name(head_arg).value_name(&head_arg.to_uppercase())),
            SubCommand::with_name(tail.name)
                .about(tail.description)
                .visible_aliases(&tail.aliases)
                .arg(Arg::with_name(tail_arg).value_name(&tail_arg.to_uppercase())),
            SubCommand::with_name(pick.name)
                .about(pick.description)
                .visible_aliases(&pick.aliases)
                .arg(
                    Arg::with_name(pick_arg)
                        .value_name(&pick_arg.to_uppercase())
                        .required(true)
                        .multiple(true),
                ),
            subcommand_for(Length),
            SubCommand::with_name(move_item.name)
                .about(move_item.description)
                .visible_aliases(&move_item.aliases)
                .arg(
                    Arg::with_name(move_item_arg)
                        .value_name(&move_item_arg.to_uppercase())
                        .required(true),
                ),
            SubCommand::with_name(move_all.name)
                .about(move_all.description)
                .visible_aliases(&move_all.aliases)
                .arg(
                    Arg::with_name(move_all_arg)
                        .value_name(&move_all_arg.to_uppercase())
                        .required(true),
                ),
            subcommand_for(IsEmpty),
            subcommand_for(Next),
            subcommand_for(Swap),
            subcommand_for(Rot),
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
    } else if let Some(matches) = command_is_opt(pick.name) {
        let indices = matches
            .values_of(pick_arg)
            .unwrap()
            .map(|i| usize::from_str_radix(&i, 10).unwrap())
            .collect();
        Pick(indices)
    } else if command_is(Complete.data().name) {
        Complete
    } else if command_is(Delete.data().name) {
        Delete
    } else if command_is(DeleteAll.data().name) {
        DeleteAll
    } else if command_is(List.data().name) {
        List
    } else if let Some(n) = command_is_opt(head.name) {
        let n = n
            .value_of(head_arg)
            // FIXME: Validate it's numeric or fail gracefully
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Head(n)
    } else if let Some(n) = command_is_opt(tail.name) {
        let n = n
            .value_of(tail_arg)
            // FIXME: Validate it's numeric or fail gracefully
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Tail(n)
    } else if let Some(dest) = command_is_opt(move_item.name) {
        let dest = dest.value_of(move_item_arg).unwrap().to_string();
        Move(dest)
    } else if let Some(dest) = command_is_opt(move_all.name) {
        let dest = dest.value_of(move_all_arg).unwrap().to_string();
        MoveAll(dest)
    } else if command_is(Length.data().name) {
        Length
    } else if command_is(IsEmpty.data().name) {
        IsEmpty
    } else if command_is(Next.data().name) {
        Next
    } else if command_is(Swap.data().name) {
        Swap
    } else if command_is(Rot.data().name) {
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
