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

    loop {
        match rl.readline(prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();

                if let Some(term) = tokens.get(0) {
                    let term = term.to_ascii_lowercase();
                    match term.as_str() {
                        "?" => Cli::command().print_help().unwrap(),
                        "help" => Cli::command().print_long_help().unwrap(),
                        "exit" | "quit" | "q" => {
                            output.log(
                                vec!["exit-reason", "exit-message"],
                                vec![vec![&term, "Buen biåhe!"]],
                            );
                            break;
                        }
                        _ => {
                            if let Some(effect) = parse_effect(tokens, stack.clone(), output) {
                                effect.run(&DEFAULT_BACKEND, &output);
                            } else {
                                output
                                    .log(vec!["term", "error"], vec![vec![&term, "unknown term"]]);
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                output.log(
                    vec!["exit-reason", "exit-message"],
                    vec![vec!["CTRL-C", "Buen biåhe!"]],
                );
                break;
            }
            Err(ReadlineError::Eof) => {
                output.log(
                    vec!["exit-reason", "exit-message"],
                    vec![vec!["CTRL-D", "Buen biåhe!"]],
                );
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
        if let Some(dest) = tokens.get(1) {
            let dest = dest.to_string();
            return Some(StackEffect::Move { stack, dest });
        }
        output.log(
            vec!["error"],
            vec![vec!["No destination stack was provided"]],
        );
        return None;
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

    if output.is_nonquiet_for_humans() {
        println!("Ooops, I don't know {:?}", term);
    } else {
        output.log(vec!["unknown-command"], vec![vec![term]]);
    };

    None
}
