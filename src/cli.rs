use crate::actions::{Action, Command};
use crate::data::Item;
use crate::effects::*;
use crate::output::{NoiseLevel, OutputFormat};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::iter;

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version of the CLI.
pub const SIGI_VERSION: &str = "1.1.0";

const DEFAULT_STACK_NAME: &str = "sigi";

fn get_app() -> App<'static, 'static> {
    let peek_names = Peek::names();

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
            SubCommand::with_name(peek_names.name)
                .about("Show the first item. (This is the default behavior when no command is given)")
                .visible_aliases(peek_names.aliases)
        )
        .subcommands(vec![
            Push::names(),
            Complete::names(),
            Delete::names(),
            DeleteAll::names(),
            ListAll::names(),
            Head::names(),
            Tail::names(),
            Pick::names(),
            Count::names(),
            Move::names(),
            MoveAll::names(),
            IsEmpty::names(),
            Next::names(),
            Swap::names(),
            Rot::names(),
        ]
        .into_iter()
        .map(subcommand_for))
}

/// Parses command line arguments and returns a single `sigi::actions::Command`.
pub fn parse_command() -> Command {
    let matches = get_app().get_matches();

    let maybe_action = |action: &Action| matches.subcommand_matches(action.data().name);
    let maybe_command = |names: EffectNames| matches.subcommand_matches(names.name);

    let action: Action = if let Some(matches) = maybe_command(Push::names()) {
        if let Some(name_bits) = matches.values_of(Push::names().input.arg_name()) {
            let name = name_bits.collect::<Vec<_>>().join(" ");
            Action::Create(Item::new(&name))
        } else {
            error_no_command(Push::names().name, matches.is_present("silent"))
        }
    } else if let Some(matches) = maybe_command(Pick::names()) {
        let indices = matches
            .values_of(Pick::names().input.arg_name())
            .unwrap()
            .map(|i| usize::from_str_radix(&i, 10).unwrap())
            .collect();
        Action::Pick(indices)
    } else if let Some(n) = maybe_command(Head::names()) {
        let n = n
            .value_of(Head::names().input.arg_name())
            .map(only_digits)
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Action::Head(n)
    } else if let Some(n) = maybe_command(Tail::names()) {
        let n = n
            .value_of(Tail::names().input.arg_name())
            .map(only_digits)
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Action::Tail(n)
    } else if let Some(dest) = maybe_command(Move::names()) {
        let dest = dest
            .value_of(Move::names().input.arg_name())
            .unwrap()
            .to_string();
        Action::Move(dest)
    } else if let Some(dest) = maybe_command(MoveAll::names()) {
        let dest = dest
            .value_of(MoveAll::names().input.arg_name())
            .unwrap()
            .to_string();
        Action::MoveAll(dest)
    } else if let Some(command) = vec![
        Action::Complete,
        Action::Delete,
        Action::DeleteAll,
        Action::List,
        Action::Length,
        Action::IsEmpty,
        Action::Next,
        Action::Swap,
        Action::Rot,
    ]
    .into_iter()
    .find(|action| maybe_action(&action).is_some())
    {
        command
    } else {
        Action::Peek
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

fn subcommand_for<'a, 'b>(names: EffectNames<'a>) -> App<'a, 'b> {
    let cmd = SubCommand::with_name(names.name)
        .about(names.description)
        .visible_aliases(names.aliases);

    if let EffectInput::NoInput = names.input {
        return cmd;
    }

    let (is_required, is_multiple) = match names.input {
        EffectInput::OptionalSingle(_) => (false, false),
        EffectInput::RequiredSingle(_) => (true, false),
        EffectInput::RequiredSlurpy(_) => (true, true),
        EffectInput::NoInput => unreachable!(),
    };

    cmd.arg(
        Arg::with_name(names.input.arg_name())
            .takes_value(true)
            .required(is_required)
            .multiple(is_multiple),
    )
}

fn error_no_command(name: &str, silent: bool) -> Action {
    if !silent {
        eprintln!("Error, not enough arguments given for: {}", name);
    }
    Action::Peek
}
