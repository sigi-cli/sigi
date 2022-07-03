use super::*;
use crate::effects::StackEffect;
use crate::output::OutputFormat;
use clap::CommandFactory;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;

const HUMAN_PROMPT: &str = "🌴 ▶ ";

pub const INTERACT_INSTRUCTIONS: &str = "INTERACTIVE MODE:

Use subcommands in interactive mode directly. \
No OPTIONS (flags) are understood in interactive mode.

The following additional commands are available:
    ?               Show the short version of \"help\"
    clear           Clear the terminal screen
    stack           Change to the specified stack
    quit/q/exit     Quit interactive mode";

pub const INTERACT_LONG_INSTRUCTIONS: &str = "INTERACTIVE MODE:

Use subcommands in interactive mode directly. For example:

    🌴 ▶ push a new thing
    Created: a new thing
    🌴 ▶ peek
    Now: a new thing
    🌴 ▶ delete
    Deleted: a new thing
    Now: nothing
    🌴 ▶ exit
    exit: Buen biåhe!

No OPTIONS (flags) are understood in interactive mode.

The following additional commands are available:
    ?
            Show the short version of \"help\"
    clear   
            Clear the terminal screen
    stack
            Change to the specified stack
    quit/q/exit
            Quit interactive mode";

// TODO: pagination/scrollback?
// TODO: more comprehensive tests
pub fn interact(original_stack: String, output: OutputFormat) {
    print_welcome_msg(output);

    let mut rl = Editor::<()>::new();
    let prompt = if output.is_nonquiet_for_humans() {
        HUMAN_PROMPT
    } else {
        ""
    };

    let mut stack = original_stack;

    loop {
        let line = rl.readline(prompt);

        if let Ok(line) = &line {
            rl.add_history_entry(line);
        }

        use ParseResult::*;
        match parse_line(line, stack.clone()) {
            ShortHelp => Cli::command().print_help().unwrap(),
            LongHelp => Cli::command().print_long_help().unwrap(),
            Clear => clearscreen::clear().expect("Failed to clear screen"),
            DoEffect(effect) => effect.run(&DEFAULT_BACKEND, &output),
            UseStack(new_stack) => {
                stack = new_stack;
                output.log(vec!["update", "stack"], vec![vec!["Active stack", &stack]]);
            }
            NoContent => (),
            Exit(reason) => {
                print_goodbye_msg(&reason, output);
                break;
            }
            MissingArgument(msg) => {
                output.log(
                    vec!["argument", "error"],
                    vec![vec![&msg, "missing argument"]],
                );
            }
            Error(msg) => {
                output.log(
                    vec!["exit-message", "exit-reason"],
                    vec![vec!["Error"], vec![&msg]],
                );
                return;
            }
            Unknown(term) => {
                if output.is_nonquiet_for_humans() {
                    println!("Oops, I don't know {:?}", term);
                } else {
                    output.log(vec!["term", "error"], vec![vec![&term, "unknown term"]]);
                };
            }
        };
    }
}

fn print_welcome_msg(output: OutputFormat) {
    if output.is_nonquiet_for_humans() {
        println!("sigi {}", SIGI_VERSION);
        println!(
            "Type \"quit\", \"q\", or \"exit\" to quit. (On Unixy systems, Ctrl+C or Ctrl+D also work)"
        );
        println!("Type \"?\" for quick help, or \"help\" for a more verbose help message.");
        println!();
    }
}

fn print_goodbye_msg(reason: &str, output: OutputFormat) {
    output.log(
        vec!["exit-reason", "exit-message"],
        vec![vec![reason, "Buen biåhe!"]],
    );
}

enum ParseResult {
    ShortHelp,
    LongHelp,
    Clear,
    DoEffect(StackEffect),
    UseStack(String),
    NoContent,
    Exit(String),
    MissingArgument(String),
    Error(String),
    Unknown(String),
}

fn parse_line(line: Result<String, ReadlineError>, stack: String) -> ParseResult {
    match line {
        Err(ReadlineError::Interrupted) => return ParseResult::Exit("Ctrl+c".to_string()),
        Err(ReadlineError::Eof) => return ParseResult::Exit("Ctrl+d".to_string()),
        Err(err) => return ParseResult::Error(format!("{:?}", err)),
        _ => (),
    };

    let line = line.unwrap();
    let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();

    if tokens.is_empty() {
        return ParseResult::NoContent;
    }

    let term = tokens.get(0).unwrap().to_ascii_lowercase();

    match term.as_str() {
        "?" => ParseResult::ShortHelp,
        "help" => ParseResult::LongHelp,
        "clear" => ParseResult::Clear,
        "exit" | "quit" | "q" => ParseResult::Exit(term),
        "stack" => match tokens.get(1) {
            Some(stack) => ParseResult::UseStack(stack.to_string()),
            None => ParseResult::MissingArgument("stack name".to_string()),
        },
        _ => match parse_effect(tokens, stack) {
            ParseEffectResult::Effect(effect) => ParseResult::DoEffect(effect),
            ParseEffectResult::NotEffect(parse_res) => parse_res,
            ParseEffectResult::Unknown => ParseResult::Unknown(term),
        },
    }
}

enum ParseEffectResult {
    Effect(StackEffect),
    NotEffect(ParseResult),
    Unknown,
}

fn parse_effect(tokens: Vec<&str>, stack: String) -> ParseEffectResult {
    let term = tokens.get(0).unwrap_or(&"");

    let parse_n = || {
        tokens
            .get(1)
            .and_then(|s| usize::from_str(s).ok())
            .unwrap_or(DEFAULT_SHORT_LIST_LIMIT)
    };

    use ParseEffectResult::*;
    use StackEffect::*;

    if COMPLETE_TERMS.contains(term) {
        return Effect(Complete { stack });
    }
    if COUNT_TERMS.contains(term) {
        return Effect(Count { stack });
    }
    if DELETE_TERMS.contains(term) {
        return Effect(Delete { stack });
    }
    if DELETE_ALL_TERMS.contains(term) {
        return Effect(DeleteAll { stack });
    }
    if HEAD_TERMS.contains(term) {
        let n = parse_n();
        return Effect(Head { stack, n });
    }
    if IS_EMPTY_TERMS.contains(term) {
        return Effect(IsEmpty { stack });
    }
    if LIST_TERMS.contains(term) {
        return Effect(ListAll { stack });
    }
    if LIST_STACKS_TERMS.contains(term) {
        return Effect(ListStacks);
    }
    if MOVE_TERMS.contains(term) {
        match tokens.get(1) {
            Some(dest) => {
                let dest = dest.to_string();
                return Effect(Move { stack, dest });
            }
            None => {
                return NotEffect(ParseResult::MissingArgument(
                    "destination stack".to_string(),
                ));
            }
        };
    }
    if MOVE_ALL_TERMS.contains(term) {
        match tokens.get(1) {
            Some(dest) => {
                let dest = dest.to_string();
                return Effect(MoveAll { stack, dest });
            }
            None => {
                return NotEffect(ParseResult::MissingArgument(
                    "destination stack".to_string(),
                ));
            }
        };
    }
    if NEXT_TERMS.contains(term) {
        return Effect(Next { stack });
    }
    if PEEK_TERMS.contains(term) {
        return Effect(Peek { stack });
    }
    if PICK_TERMS.contains(term) {
        let indices = tokens
            .iter()
            .filter_map(|s| usize::from_str(s).ok())
            .collect();
        return Effect(Pick { stack, indices });
    }
    if PUSH_TERMS.contains(term) {
        // FIXME: This is convenient, but normalizes whitespace. (E.g. multiple spaces always collapsed, tabs to spaces, etc)
        let content = tokens[1..].join(" ");
        return Effect(Push { stack, content });
    }
    if ROT_TERMS.contains(term) {
        return Effect(Rot { stack });
    }
    if SWAP_TERMS.contains(term) {
        return Effect(Swap { stack });
    }
    if TAIL_TERMS.contains(term) {
        let n = parse_n();
        return Effect(Tail { stack, n });
    }

    Unknown
}
