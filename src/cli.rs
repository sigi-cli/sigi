use crate::data::Item;
use crate::effects::*;
use crate::output::{NoiseLevel, OutputFormat};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::iter;

// TODO: Get the version from Cargo.toml? (If possible, at compile time)
/// The current version of the CLI.
pub const SIGI_VERSION: &str = "2.1.0";

const DEFAULT_STACK_NAME: &str = "sigi";
const DEFAULT_FORMAT: OutputFormat = OutputFormat::Human(NoiseLevel::Normal);

fn get_app() -> App<'static, 'static> {
    let peek_names = Peek::names();

    let app = App::new("sigi")
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
        .subcommand(
            SubCommand::with_name(peek_names.name)
                .about(
                    "Show the first item. (This is the default behavior when no command is given)",
                )
                .visible_aliases(peek_names.aliases),
        )
        .subcommands(
            vec![
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
            .map(subcommand_for)
            .map(with_formatting_flags),
        );

    with_formatting_flags(app)
}

/// Parses command line arguments and executes a single `sigi::effects::StackEffect`.
pub fn run() {
    let matches = get_app().get_matches();

    let stack = matches
        .value_of("stack")
        .unwrap_or(DEFAULT_STACK_NAME)
        .to_string();

    let (effect, maybe_format) = get_push_effect(&stack, &matches)
        .or_else(|| get_head_effect(&stack, &matches))
        .or_else(|| get_tail_effect(&stack, &matches))
        .or_else(|| get_move_effect(&stack, &matches))
        .or_else(|| get_move_all_effect(&stack, &matches))
        .or_else(|| get_pick_effect(&stack, &matches))
        .or_else(|| get_noarg_effect(&stack, &matches))
        .unwrap_or_else(|| just_peek_effect(&stack, &matches));

    // Format settings of a subcommand take precedence over main command.
    let output = maybe_format
        .or_else(|| get_format(&matches))
        .unwrap_or(DEFAULT_FORMAT);

    effect.run(output);
}

fn get_format(matches: &ArgMatches) -> Option<OutputFormat> {
    if matches.is_present("verbose") {
        Some(OutputFormat::Human(NoiseLevel::Verbose))
    } else if matches.is_present("silent") {
        Some(OutputFormat::Silent)
    } else if matches.is_present("quiet") {
        Some(OutputFormat::Human(NoiseLevel::Quiet))
    } else if let Some(fmt) = matches.value_of("format") {
        match fmt {
            "csv" => Some(OutputFormat::Csv),
            "json" => Some(OutputFormat::Json),
            "json-compact" => Some(OutputFormat::JsonCompact),
            "tsv" => Some(OutputFormat::Tsv),
            _ => None,
        }
    } else {
        None
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

fn with_formatting_flags<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app
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
            .help("Use a programmatic format. Options include [csv, json, json-compact, tsv]. Not compatible with quiet/silent/verbose.")
    )
}

// ===== Clap compat =====

fn get_push_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    let names = Push::names();

    let push_matches = matches.subcommand_matches(&names.name);

    push_matches
        .map(|matches| matches.values_of(names.input.arg_name()).unwrap())
        .map(|contents| {
            let contents = contents.collect::<Vec<_>>().join(" ");
            let item = Item::new(&contents);

            let push: Box<dyn StackEffect> = Box::new(Push {
                stack: stack.to_string(),
                item,
            });

            push
        })
        .map(|push| (push, push_matches.and_then(get_format)))
}

fn get_pick_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    let pick_matches = matches.subcommand_matches(&Pick::names().name);

    pick_matches
        .map(|matches| {
            let indices = matches
                .values_of(Pick::names().input.arg_name())
                .unwrap()
                .map(|i| usize::from_str_radix(&i, 10).unwrap())
                .collect();

            let pick: Box<dyn StackEffect> = Box::new(Pick {
                stack: stack.to_string(),
                indices,
            });

            pick
        })
        .map(|pick| (pick, pick_matches.and_then(get_format)))
}

fn get_head_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    let names = Head::names();
    let head_matches = matches.subcommand_matches(names.name);

    head_matches
        .map(|matches| {
            let n = matches
                .value_of(names.input.arg_name())
                .map(only_digits)
                .map(|i| usize::from_str_radix(&i, 10).ok())
                .flatten();

            let head: Box<dyn StackEffect> = Box::new(Head {
                stack: stack.to_string(),
                n,
            });
            head
        })
        .map(|head| (head, head_matches.and_then(get_format)))
}

fn get_tail_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    let names = Tail::names();
    let tail_matches = matches.subcommand_matches(names.name);

    tail_matches
        .map(|matches| {
            let n = matches
                .value_of(names.input.arg_name())
                .map(only_digits)
                .map(|i| usize::from_str_radix(&i, 10).ok())
                .flatten();
            let tail: Box<dyn StackEffect> = Box::new(Tail {
                stack: stack.to_string(),
                n,
            });
            tail
        })
        .map(|tail| (tail, tail_matches.and_then(get_format)))
}

fn get_move_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    let names = Move::names();
    let move_matches = matches.subcommand_matches(names.name);

    move_matches
        .map(|matches| {
            let dest_stack = matches
                .value_of(names.input.arg_name())
                .unwrap()
                .to_string();
            let move_: Box<dyn StackEffect> = Box::new(Move {
                stack: stack.to_string(),
                dest_stack,
            });
            move_
        })
        .map(|move_| (move_, move_matches.and_then(get_format)))
}

fn get_move_all_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    let names = MoveAll::names();
    let move_all_matches = matches.subcommand_matches(names.name);

    move_all_matches
        .map(|matches| {
            let dest_stack = matches
                .value_of(names.input.arg_name())
                .unwrap()
                .to_string();
            let move_all: Box<dyn StackEffect> = Box::new(MoveAll {
                stack: stack.to_string(),
                dest_stack,
            });
            move_all
        })
        .map(|move_all| (move_all, move_all_matches.and_then(get_format)))
}

fn get_noarg_effect(
    stack: &str,
    matches: &ArgMatches,
) -> Option<(Box<dyn StackEffect>, Option<OutputFormat>)> {
    // TODO: How can I avoid allocating all this?
    let candidates: Vec<(EffectNames, Box<dyn StackEffect>)> = vec![
        (Complete::names(), Box::new(Complete::from(stack))),
        (Delete::names(), Box::new(Delete::from(stack))),
        (DeleteAll::names(), Box::new(DeleteAll::from(stack))),
        (ListAll::names(), Box::new(ListAll::from(stack))),
        (Count::names(), Box::new(Count::from(stack))),
        (IsEmpty::names(), Box::new(IsEmpty::from(stack))),
        (Next::names(), Box::new(Next::from(stack))),
        (Swap::names(), Box::new(Swap::from(stack))),
        (Rot::names(), Box::new(Rot::from(stack))),
    ];

    candidates
        .into_iter()
        .map(|(names, effect)| (effect, matches.subcommand_matches(names.name)))
        .find(|(_, effect_matches)| effect_matches.is_some())
        .map(|(effect, effect_matches)| (effect, effect_matches.and_then(get_format)))
}

fn just_peek_effect(
    stack: &str,
    matches: &ArgMatches,
) -> (Box<dyn StackEffect>, Option<OutputFormat>) {
    let peek = Box::new(Peek {
        stack: stack.to_string(),
    });

    let matches = matches.subcommand_matches(Peek::names().name);

    let format = matches.and_then(get_format);

    (peek, format)
}
