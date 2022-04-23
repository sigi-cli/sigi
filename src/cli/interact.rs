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
    quit/q/exit
            Quit interactive mode";

// TODO: clear (i.e. clear screen)
// TODO: change-stack (i.e. change working stack)
// TODO: pagination/scrollback?
// TODO: tests
// TODO: refactor & clean
pub fn interact(stack: String, output: OutputFormat) {
    if output.is_nonquiet_for_humans() {
        println!("sigi {}", SIGI_VERSION);
        println!(
            "Type \"quit\", \"q\", or \"exit\" to quit. (On Unixy systems, Ctrl+C or Ctrl+D also work)"
        );
        println!("Type \"?\" for quick help, or \"help\" for a more verbose help message.");
        println!();
    };

    let mut rl = Editor::<()>::new();
    let prompt = if output.is_nonquiet_for_humans() {
        HUMAN_PROMPT
    } else {
        ""
    };

    let print_goodbye_msg = |reason| {
        output.log(
            vec!["exit-reason", "exit-message"],
            vec![vec![reason, "Buen biåhe!"]],
        )
    };

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                use ParseResult::*;
                match parse_line(&line, stack.clone(), output) {
                    ShortHelp => Cli::command().print_help().unwrap(),
                    LongHelp => Cli::command().print_long_help().unwrap(),
                    Exit(term) => {
                        print_goodbye_msg(&term);
                        break;
                    }
                    DoEffect(effect) => effect.run(&DEFAULT_BACKEND, &output),
                    NoContent => (),
                    Unknown(term) => {
                        if output.is_nonquiet_for_humans() {
                            println!("Oops, I don't know {:?}", term);
                        } else {
                            output.log(vec!["term", "error"], vec![vec![&term, "unknown term"]]);
                        };
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                print_goodbye_msg("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                print_goodbye_msg("CTRL-D");
                break;
            }
            Err(err) => {
                output.log(
                    vec!["exit-message", "exit-reason"],
                    vec![vec!["Error"], vec![&format!("{:?}", err)]],
                );
                std::process::exit(1);
            }
        }
    }
}

enum ParseResult {
    ShortHelp,
    LongHelp,
    Exit(String),
    DoEffect(StackEffect),
    NoContent,
    Unknown(String),
}

fn parse_line(line: &str, stack: String, output: OutputFormat) -> ParseResult {
    let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();

    if tokens.is_empty() {
        return ParseResult::NoContent;
    }

    let term = tokens.get(0).unwrap().to_ascii_lowercase();

    match term.as_str() {
        "?" => ParseResult::ShortHelp,
        "help" => ParseResult::LongHelp,
        "exit" | "quit" | "q" => ParseResult::Exit(term),
        _ => match parse_effect(tokens, stack, output) {
            Some(effect) => ParseResult::DoEffect(effect),
            None => ParseResult::Unknown(term),
        },
    }
}

fn parse_effect(tokens: Vec<&str>, stack: String, output: OutputFormat) -> Option<StackEffect> {
    let term = tokens.get(0).unwrap_or(&"");

    let parse_n = || {
        tokens
            .get(1)
            .map(|s| usize::from_str(s).ok())
            .flatten()
            .unwrap_or(DEFAULT_SHORT_LIST_LIMIT)
    };

    if COMPLETE_TERMS.contains(term) {
        return Some(StackEffect::Complete { stack });
    }
    if COUNT_TERMS.contains(term) {
        return Some(StackEffect::Count { stack });
    }
    if DELETE_TERMS.contains(term) {
        return Some(StackEffect::Delete { stack });
    }
    if DELETE_ALL_TERMS.contains(term) {
        return Some(StackEffect::DeleteAll { stack });
    }
    if HEAD_TERMS.contains(term) {
        let n = parse_n();
        return Some(StackEffect::Head { stack, n });
    }
    if IS_EMPTY_TERMS.contains(term) {
        return Some(StackEffect::IsEmpty { stack });
    }
    if LIST_TERMS.contains(term) {
        return Some(StackEffect::ListAll { stack });
    }
    if LIST_STACKS_TERMS.contains(term) {
        return Some(StackEffect::ListStacks);
    }
    if &MOVE_TERM == term {
        match tokens.get(1) {
            Some(dest) => {
                let dest = dest.to_string();
                return Some(StackEffect::Move { stack, dest });
            }
            None => {
                output.log(
                    vec!["error"],
                    vec![vec!["No destination stack was provided"]],
                );
                return None;
            }
        };
    }
    if &MOVE_ALL_TERM == term {
        if let Some(dest) = tokens.get(1) {
            let dest = dest.to_string();
            return Some(StackEffect::MoveAll { stack, dest });
        }
        output.log(
            vec!["error"],
            vec![vec!["No destination stack was provided"]],
        );
        return None;
    }
    if NEXT_TERMS.contains(term) {
        return Some(StackEffect::Next { stack });
    }
    if PEEK_TERMS.contains(term) {
        return Some(StackEffect::Peek { stack });
    }
    if &PICK_TERM == term {
        let indices = tokens
            .iter()
            .map(|s| usize::from_str(s).ok())
            .flatten()
            .collect();
        return Some(StackEffect::Pick { stack, indices });
    }
    if PUSH_TERMS.contains(term) {
        // FIXME: This is convenient, but normalizes whitespace. (E.g. multiple spaces always collapsed, tabs to spaces, etc)
        let content = tokens[1..].join(" ");
        return Some(StackEffect::Push { stack, content });
    }
    if ROT_TERMS.contains(term) {
        return Some(StackEffect::Rot { stack });
    }
    if &SWAP_TERM == term {
        return Some(StackEffect::Swap { stack });
    }
    if TAIL_TERMS.contains(term) {
        let n = parse_n();
        return Some(StackEffect::Tail { stack, n });
    }

    None
}
