use super::*;
use crate::effects::StackEffect;
use crate::output::OutputFormat;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;

// TODO: help/?, q/quit/exit
// TODO: clear (i.e. clear screen)
// TODO: change-stack (i.e. change working stack)
// TODO: pagination/scrollback?
// TODO: tests
pub fn interact(stack: String, output: OutputFormat) {
    println!("sigi {}", SIGI_VERSION);
    let mut rl = Editor::<()>::new();
    loop {
        let readline = match output {
            OutputFormat::Human(_) => rl.readline("🌴 ▶️ "),
            _ => rl.readline(""),
        };
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let tokens = line.split_ascii_whitespace().collect();
                if let Some(effect) = parse_effect(tokens, stack.clone(), output) {
                    effect.run(&DEFAULT_BACKEND, &output);
                }
            }
            Err(ReadlineError::Interrupted) => {
                output.log(
                    vec!["exit-message", "reason"],
                    vec![vec!["Buen biåhe", "CTRL-C"]],
                );
                break;
            }
            Err(ReadlineError::Eof) => {
                output.log(
                    vec!["exit-message", "reason"],
                    vec![vec!["Buen biåhe", "CTRL-D"]],
                );
                break;
            }
            Err(err) => {
                output.log(
                    vec!["exit-message", "reason"],
                    vec![vec!["Error"], vec![&format!("{:?}", err)]],
                );
                break;
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

    match output {
        OutputFormat::Human(_) => println!("Ooops, I don't know {:?}", term),
        _ => output.log(vec!["unknown-command"], vec![vec![term]]),
    };

    None
}
