use crate::data::Backend;
use crate::effects::{push_item, Head, StackAction};
use crate::output::OutputFormat;

// ===== Pick =====

/// Move the specified indices to the top of stack.
pub struct Pick {
    pub stack: String,
    pub indices: Vec<usize>,
}

impl StackAction for Pick {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        if let Ok(stack) = backend.load(&self.stack) {
            let mut stack = stack;
            let mut seen: Vec<usize> = vec![];
            seen.reserve_exact(self.indices.len());
            let indices: Vec<usize> = self
                .indices
                .iter()
                .map(|i| stack.len() - 1 - i)
                .rev()
                .collect();
            for i in indices {
                if i > stack.len() || seen.contains(&i) {
                    // TODO: What should be the output here? Some stderr?
                    // command.log("Pick", "ignoring out-of-bounds index");
                    // command.log("Pick", "ignoring duplicate index");
                    continue;
                }
                let i = i - seen.iter().filter(|j| j < &&i).count();
                let picked = stack.remove(i);
                stack.push(picked);
                seen.push(i);
            }

            backend.save(&self.stack, stack).unwrap();

            let head = Head {
                stack: self.stack.clone(),
                n: Some(seen.len()),
            };

            head.run(backend, output);
        }
    }
}

// ===== Move =====

/// Move the current item to a different stack.
pub struct Move {
    pub stack: String,
    pub dest: String,
}

impl StackAction for Move {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        if let Ok(items) = backend.load(&self.stack) {
            let mut items = items;
            if let Some(item) = items.pop() {
                backend.save(&self.stack, items).unwrap();

                output.log(
                    vec!["action", "new-stack", "old-stack"],
                    vec![vec!["Move", &self.dest, &self.stack]],
                );

                push_item(self.dest, item, backend, &OutputFormat::Silent);
            }
        }
    }
}

// ===== MoveAll =====

/// Move all items to a different stack.
pub struct MoveAll {
    pub stack: String,
    pub dest: String,
}

impl StackAction for MoveAll {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        if let Ok(src_items) = backend.load(&self.stack) {
            let count = src_items.len();

            if !src_items.is_empty() {
                let all_items = match backend.load(&self.dest) {
                    Ok(dest_items) => {
                        let mut all_items = dest_items;
                        for item in src_items {
                            all_items.push(item);
                        }
                        all_items
                    }
                    _ => src_items,
                };

                backend.save(&self.dest, all_items).unwrap();
                backend.save(&self.stack, vec![]).unwrap();
            }

            output.log(
                vec!["action", "new-stack", "old-stack", "num-moved"],
                vec![vec![
                    "Move All",
                    &self.dest,
                    &self.stack,
                    &count.to_string(),
                ]],
            );
        }
    }
}
