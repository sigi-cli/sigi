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

/// Parses command line arguments and executes a single `sigi::effects::StackEffect`.
pub fn run() {
    let matches = get_app().get_matches();

    let stack = matches
        .value_of("stack")
        .unwrap_or(DEFAULT_STACK_NAME)
        .to_string();

    let effect = get_effect(stack, &matches);

    let output = get_format(matches);

    effect.run(output);
}

fn get_effect(stack: String, matches: &ArgMatches) -> Box<dyn StackEffect> {
    let maybe_effect = |names: EffectNames| matches.subcommand_matches(names.name);

    if let Some(matches) = maybe_effect(Push::names()) {
        if let Some(name_bits) = matches.values_of(Push::names().input.arg_name()) {
            let contents = name_bits.collect::<Vec<_>>().join(" ");
            let item = Item::new(&contents);

            Box::new(Push { stack, item })
        } else {
            error_no_command(Push::names().name, matches.is_present("silent"));
            Box::new(Peek { stack })
        }
    } else if let Some(matches) = maybe_effect(Pick::names()) {
        let indices = matches
            .values_of(Pick::names().input.arg_name())
            .unwrap()
            .map(|i| usize::from_str_radix(&i, 10).unwrap())
            .collect();
        Box::new(Pick { stack, indices })
    } else if let Some(n) = maybe_effect(Head::names()) {
        let n = n
            .value_of(Head::names().input.arg_name())
            .map(only_digits)
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Box::new(Head { stack, n })
    } else if let Some(n) = maybe_effect(Tail::names()) {
        let n = n
            .value_of(Tail::names().input.arg_name())
            .map(only_digits)
            .map(|i| usize::from_str_radix(&i, 10).ok())
            .flatten();
        Box::new(Tail { stack, n })
    } else if let Some(dest) = maybe_effect(Move::names()) {
        let dest_stack = dest
            .value_of(Move::names().input.arg_name())
            .unwrap()
            .to_string();
        Box::new(Move { stack, dest_stack })
    } else if let Some(dest) = maybe_effect(MoveAll::names()) {
        let dest_stack = dest
            .value_of(MoveAll::names().input.arg_name())
            .unwrap()
            .to_string();
        Box::new(MoveAll { stack, dest_stack })
    } else if maybe_effect(Complete::names()).is_some() {
        Box::new(Complete { stack })
    } else if maybe_effect(Delete::names()).is_some() {
        Box::new(Delete { stack })
    } else if maybe_effect(DeleteAll::names()).is_some() {
        Box::new(DeleteAll { stack })
    } else if maybe_effect(ListAll::names()).is_some() {
        Box::new(ListAll { stack })
    } else if maybe_effect(Count::names()).is_some() {
        Box::new(Count { stack })
    } else if maybe_effect(IsEmpty::names()).is_some() {
        Box::new(IsEmpty { stack })
    } else if maybe_effect(Next::names()).is_some() {
        Box::new(Next { stack })
    } else if maybe_effect(Swap::names()).is_some() {
        Box::new(Swap { stack })
    } else if maybe_effect(Rot::names()).is_some() {
        Box::new(Rot { stack })
    } else {
        Box::new(Peek { stack })
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

fn error_no_command(name: &str, silent: bool) {
    if !silent {
        eprintln!("Error, not enough arguments given for: {}", name);
    }
}
