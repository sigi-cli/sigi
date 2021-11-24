use crate::data;
use crate::output::OutputFormat;
use chrono::{DateTime, Local};

const HISTORY_SUFFIX: &str = "_history";

pub trait StackEffect {
    fn names<'a>() -> EffectNames<'a>;
    fn run(&self, output: OutputFormat);
}

pub struct EffectNames<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub aliases: &'a [&'a str],
}

// ===== Peek =====

pub struct Peek {
    pub stack: String,
}

impl StackEffect for Peek {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "peek",
            description: "Show the current item",
            aliases: &["show"],
        }
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let top_item = &items.last().unwrap().contents;
            if !items.is_empty() {
                output.log(vec!["position", "item"], vec![vec!["Now", top_item]]);
            }
        }
    }
}

// ===== Create (Push) =====

pub struct Push {
    pub stack: String,
    pub item: data::Item,
}

impl StackEffect for Push {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "push",
            description: "Create a new item",
            aliases: &["create", "add", "do", "start", "new"],
        }
    }

    fn run(&self, output: OutputFormat) {
        let new_items = if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            items.push(self.item.clone());
            items
        } else {
            vec![self.item.clone()]
        };
        data::save(&self.stack, new_items).unwrap();
        output.log(
            vec!["action", "item"],
            vec![vec!["Created", &self.item.contents]],
        );
    }
}

// ===== Complete (Pop) =====

pub struct Complete {
    pub stack: String,
}

impl StackEffect for Complete {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "complete",
            description: "Move the current item to \"<STACK>_history\" and mark as completed.",
            aliases: &["done", "finish", "fulfill"],
        }
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            if let Some(item) = items.pop() {
                let mut item = item;
                item.mark_completed();

                // Push the now-marked-completed item to history stack.
                Push {
                    stack: stack_history_of(&self.stack),
                    item: item.clone(),
                }
                .run(OutputFormat::Silent);

                // Save the original stack without that item.
                data::save(&self.stack, items).unwrap();

                output.log(
                    vec!["action", "item"],
                    vec![vec!["Completed", &item.contents]],
                );

                // Peek the current stack only for human output.
                if let OutputFormat::Human(_) = output {
                    Peek {
                        stack: self.stack.clone(),
                    }
                    .run(output);
                }
            }
        }
    }
}

// ===== Delete (Pop) =====

pub struct Delete {
    pub stack: String,
}

impl StackEffect for Delete {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "delete",
            description: "Move the current item to \"<STACK>_history\" and mark as deleted.",
            aliases: &["pop", "remove", "cancel", "drop"],
        }
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            if let Some(item) = items.pop() {
                let mut item = item;
                item.mark_deleted();

                // Push the now-marked-deleted item to history stack.
                Push {
                    stack: stack_history_of(&self.stack),
                    item: item.clone(),
                }
                .run(OutputFormat::Silent);

                // Save the original stack without that item.
                data::save(&self.stack, items).unwrap();

                output.log(
                    vec!["action", "item"],
                    vec![vec!["Deleted", &item.contents]],
                );

                // Peek the current stack only for human output.
                if let OutputFormat::Human(_) = output {
                    Peek {
                        stack: self.stack.clone(),
                    }
                    .run(output);
                }
            }
        }
    }
}

// ===== DeleteAll (Pop all) =====

pub struct DeleteAll {
    pub stack: String,
}

impl StackEffect for DeleteAll {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "delete-all",
            description: "Move all items to \"<STACK>_history\" and mark as deleted.",
            aliases: &["purge", "pop-all", "remove-all", "cancel-all", "drop-all"],
        }
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            items.iter_mut().for_each(|item| item.mark_deleted());

            // Push the now-marked-deleted items to history stack.
            let history_stack = &stack_history_of(&self.stack);
            let mut history = data::load(history_stack).unwrap_or(vec![]);
            history.append(&mut items);
            data::save(history_stack, history).unwrap();

            // Save the original stack as empty now.
            data::save(&self.stack, vec![]).unwrap();

            output.log(
                vec!["action", "item"],
                vec![vec!["Deleted", &format!("{} items", items.len())]],
            );
        }
    }
}

// ===== Some help for doing ListAll/Head/Tail =====

trait Listable {
    fn range<'a>(&'a self) -> ListRange<'a>;
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
            .enumerate()
            .skip(start)
            .take(limit)
            .map(|(i, item)| {
                let position = match output {
                    // Pad human output numbers to line up nicely with "Now".
                    OutputFormat::Human(_) => match i {
                        0 => "Now".to_string(),
                        1..=9 => format!("  {}", i),
                        10..=099 => format!(" {}", i),
                        _ => i.to_string(),
                    },
                    _ => i.to_string(),
                };

                let created = item
                    .history
                    .iter()
                    .find(|(status, _)| status == "created")
                    .map(|(_, dt)| dt.to_string())
                    .unwrap_or("unknown".to_string());

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

pub struct ListAll {
    pub stack: String,
}

impl Listable for ListAll {
    fn range<'a>(&'a self) -> ListRange<'a> {
        ListRange {
            stack: &self.stack,
            start: 0,
            limit: None,
            from_end: false,
        }
    }
}

impl StackEffect for ListAll {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "list",
            description: "List all items",
            aliases: &["ls", "snoop", "show", "all"],
        }
    }

    fn run(&self, output: OutputFormat) {
        list_range(self, output);
    }
}

// ===== Head =====

const HEAD_DEFAULT_LIMIT: usize = 10;

pub struct Head {
    pub stack: String,
    pub n: Option<usize>,
}

impl Listable for Head {
    fn range<'a>(&'a self) -> ListRange<'a> {
        ListRange {
            stack: &self.stack,
            start: 0,
            limit: Some(self.n.unwrap_or(HEAD_DEFAULT_LIMIT)),
            from_end: false,
        }
    }
}

impl StackEffect for Head {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "head",
            description: "List the first N items",
            aliases: &["top", "first"],
        }
    }

    fn run(&self, output: OutputFormat) {
        list_range(self, output);
    }
}

// ===== Tail =====

const TAIL_DEFAULT_LIMIT: usize = 10;

pub struct Tail {
    pub stack: String,
    pub n: Option<usize>,
}

impl Listable for Tail {
    fn range<'a>(&'a self) -> ListRange<'a> {
        ListRange {
            stack: &self.stack,
            start: 0,
            limit: Some(self.n.unwrap_or(TAIL_DEFAULT_LIMIT)),
            from_end: true,
        }
    }
}

impl StackEffect for Tail {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "tail",
            description: "List the last N items",
            aliases: &["bottom", "last"],
        }
    }

    fn run(&self, output: OutputFormat) {
        list_range(self, output);
    }
}

// ===== Count =====

pub struct Count {
    pub stack: String,
}

impl StackEffect for Count {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "count",
            description: "Print the total number of items in the stack",
            aliases: &["size", "length"],
        }
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let len = items.len().to_string();
            output.log(vec!["items"], vec![vec![&len]])
        }
    }
}

// ===== IsEmpty =====

pub struct IsEmpty {
    pub stack: String,
}

impl StackEffect for IsEmpty {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "is-empty",
            description: "\"true\" if stack has zero items, \"false\" (and nonzero exit code) if the stack does have items",
            aliases: &["empty"],
        }
    }

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

// ===== Helper functions =====

fn stack_history_of(stack: &str) -> String {
    stack.to_string() + HISTORY_SUFFIX
}

fn _format_time_for_humans(dt: DateTime<Local>) -> String {
    // TODO: Does this work for all locales?
    dt.to_rfc2822()
}
