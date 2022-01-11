use crate::data;
use crate::effects::{EffectInput, EffectNames, NamedEffect, StackEffect};
use crate::output::OutputFormat;

// ===== Peek =====

/// Look at the most-recent item.
pub struct Peek {
    pub stack: String,
}

impl NamedEffect for Peek {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "peek",
            description: "Show the current item",
            aliases: &["show"],
            input: EffectInput::NoInput,
        }
    }
}

impl StackEffect for Peek {
    fn run(&self, output: OutputFormat) {
        if let OutputFormat::Silent = output {
            return;
        }

        if let Ok(items) = data::load(&self.stack) {
            let top_item = items.last().map(|i| i.contents.as_str());

            let output_it = |it| output.log(vec!["position", "item"], it);

            match top_item {
                Some(contents) => output_it(vec![vec!["Now", contents]]),
                None => match output {
                    OutputFormat::Human(_) => output_it(vec![vec!["Now", "NOTHING"]]),
                    _ => output_it(vec![]),
                },
            }
        }
    }
}

impl From<&str> for Peek {
    fn from(stack: &str) -> Peek {
        Peek {
            stack: stack.to_string(),
        }
    }
}

// ===== Some help for doing ListAll/Head/Tail =====

trait Listable {
    fn range(&self) -> ListRange;
}

struct ListRange<'a> {
    stack: &'a str,
    // Ignored if starting "from_end".
    start: usize,
    limit: Option<usize>,
    from_end: bool,
}

fn list_range(listable: &impl Listable, output: OutputFormat) {
    if let OutputFormat::Silent = output {
        return;
    }

    let range = listable.range();

    if let Ok(items) = data::load(range.stack) {
        let limit = match range.limit {
            Some(n) => n,
            None => items.len(),
        };

        let start = if range.from_end {
            if limit <= items.len() {
                items.len() - limit
            } else {
                0
            }
        } else {
            range.start
        };

        let lines = items
            .into_iter()
            .rev()
            .enumerate()
            .skip(start)
            .take(limit)
            .map(|(i, item)| {
                let position = match output {
                    // Pad human output numbers to line up nicely with "Now".
                    OutputFormat::Human(_) => match i {
                        0 => "Now".to_string(),
                        1..=9 => format!("  {}", i),
                        10..=99 => format!(" {}", i),
                        _ => i.to_string(),
                    },
                    _ => i.to_string(),
                };

                let created = item
                    .history
                    .iter()
                    .find(|(status, _)| status == "created")
                    .map(|(_, dt)| output.format_time(*dt))
                    .unwrap_or_else(|| "unknown".to_string());

                vec![position, item.contents, created]
            })
            .collect::<Vec<_>>();

        // Get the lines into a "borrow" state (&str instead of String) to make log happy.
        let lines = lines
            .iter()
            .map(|line| line.iter().map(|s| s.as_str()).collect())
            .collect();

        output.log(vec!["position", "item", "created"], lines);
    }
}

// ===== ListAll =====

/// List the stack's items.
pub struct ListAll {
    pub stack: String,
}

impl Listable for ListAll {
    fn range(&self) -> ListRange {
        ListRange {
            stack: &self.stack,
            start: 0,
            limit: None,
            from_end: false,
        }
    }
}

impl NamedEffect for ListAll {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "list",
            description: "List all items",
            aliases: &["ls", "snoop", "show", "all"],
            input: EffectInput::NoInput,
        }
    }
}

impl StackEffect for ListAll {
    fn run(&self, output: OutputFormat) {
        list_range(self, output);
    }
}

impl From<&str> for ListAll {
    fn from(stack: &str) -> ListAll {
        ListAll {
            stack: stack.to_string(),
        }
    }
}

// ===== Head =====

/// List the first N stack items.
const HEAD_DEFAULT_LIMIT: usize = 10;

pub struct Head {
    pub stack: String,
    pub n: Option<usize>,
}

impl Listable for Head {
    fn range(&self) -> ListRange {
        ListRange {
            stack: &self.stack,
            start: 0,
            limit: Some(self.n.unwrap_or(HEAD_DEFAULT_LIMIT)),
            from_end: false,
        }
    }
}

impl NamedEffect for Head {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "head",
            description: "List the first N items",
            aliases: &["top", "first"],
            input: EffectInput::OptionalSingle("n"),
        }
    }
}

impl StackEffect for Head {
    fn run(&self, output: OutputFormat) {
        list_range(self, output);
    }
}

// ===== Tail =====

/// List the last N stack items.
const TAIL_DEFAULT_LIMIT: usize = 10;

pub struct Tail {
    pub stack: String,
    pub n: Option<usize>,
}

impl Listable for Tail {
    fn range(&self) -> ListRange {
        ListRange {
            stack: &self.stack,
            start: 0,
            limit: Some(self.n.unwrap_or(TAIL_DEFAULT_LIMIT)),
            from_end: true,
        }
    }
}

impl NamedEffect for Tail {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "tail",
            description: "List the last N items",
            aliases: &["bottom", "last"],
            input: EffectInput::OptionalSingle("n"),
        }
    }
}

impl StackEffect for Tail {
    fn run(&self, output: OutputFormat) {
        list_range(self, output);
    }
}

// ===== Count =====

/// Count the stack's items.
pub struct Count {
    pub stack: String,
}

impl NamedEffect for Count {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "count",
            description: "Print the total number of items in the stack",
            aliases: &["size", "length"],
            input: EffectInput::NoInput,
        }
    }
}

impl StackEffect for Count {
    fn run(&self, output: OutputFormat) {
        if let OutputFormat::Silent = output {
            return;
        }

        if let Ok(items) = data::load(&self.stack) {
            let len = items.len().to_string();
            output.log(vec!["items"], vec![vec![&len]])
        }
    }
}

impl From<&str> for Count {
    fn from(stack: &str) -> Count {
        Count {
            stack: stack.to_string(),
        }
    }
}

// ===== IsEmpty =====

/// Determine if the stack is empty or not.
pub struct IsEmpty {
    pub stack: String,
}

impl NamedEffect for IsEmpty {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "is-empty",
            description: "Print \"true\" if stack has zero items, or print \"false\" (and exit with a nonzero exit code) if the stack does have items",
            aliases: &["empty"],
            input: EffectInput::NoInput
        }
    }
}

impl StackEffect for IsEmpty {
    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            if !items.is_empty() {
                output.log(vec!["empty"], vec![vec!["false"]]);
                // Exit with a failure (nonzero status) when not empty.
                // This helps people who do shell scripting do something like:
                //     while ! sigi -t $stack is-empty ; do <ETC> ; done
                // TODO: It would be better modeled as an error, if anyone uses as a lib this will surprise.
                std::process::exit(1);
            }
        }
        output.log(vec!["empty"], vec![vec!["true"]]);
    }
}

impl From<&str> for IsEmpty {
    fn from(stack: &str) -> IsEmpty {
        IsEmpty {
            stack: stack.to_string(),
        }
    }
}
