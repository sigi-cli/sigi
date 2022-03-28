use crate::data::Backend;
use crate::effects::StackAction;
use crate::output::OutputFormat;

// ===== Some help for doing ListAll/Head/Tail =====

trait Listable {
    fn range(self) -> ListRange;
}

struct ListRange {
    stack: String,
    // Ignored if starting "from_end".
    start: usize,
    limit: Option<usize>,
    from_end: bool,
}

fn list_range(listable: impl Listable, backend: &Backend, output: &OutputFormat) {
    if let OutputFormat::Silent = output {
        return;
    }

    let range = listable.range();

    if let Ok(items) = backend.load(&range.stack) {
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

        let labels = vec!["position", "item", "created"];

        if lines.is_empty() {
            if let OutputFormat::Human(_) = output {
                output.log(labels, vec![vec!["Now", "NOTHING"]]);
            }
            return;
        }

        // Get the lines into a "borrow" state (&str instead of String) to make log happy.
        let lines = lines
            .iter()
            .map(|line| line.iter().map(|s| s.as_str()).collect())
            .collect();

        output.log(labels, lines);
    }
}

// ===== ListAll =====

/// List the stack's items.
pub struct ListAll {
    pub stack: String,
}

impl Listable for ListAll {
    fn range(self) -> ListRange {
        ListRange {
            stack: self.stack,
            start: 0,
            limit: None,
            from_end: false,
        }
    }
}

impl StackAction for ListAll {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        list_range(self, backend, output);
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
    fn range(self) -> ListRange {
        ListRange {
            stack: self.stack,
            start: 0,
            limit: Some(self.n.unwrap_or(HEAD_DEFAULT_LIMIT)),
            from_end: false,
        }
    }
}

impl StackAction for Head {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        list_range(self, backend, output);
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
    fn range(self) -> ListRange {
        ListRange {
            stack: self.stack,
            start: 0,
            limit: Some(self.n.unwrap_or(TAIL_DEFAULT_LIMIT)),
            from_end: true,
        }
    }
}

impl StackAction for Tail {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        list_range(self, backend, output);
    }
}
