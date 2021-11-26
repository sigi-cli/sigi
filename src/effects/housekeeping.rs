use crate::data;
use crate::effects::{EffectInput, EffectNames, Head, NamedEffect, Push, StackEffect};
use crate::output::OutputFormat;

// ===== Pick =====

/// Move the specified indices to the top of stack.
pub struct Pick {
    pub stack: String,
    pub indices: Vec<usize>,
}

impl NamedEffect for Pick {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "pick",
            description: "Move items to the top of stack by their number",
            aliases: &[],
            input: EffectInput::RequiredSlurpy("number"),
        }
    }
}

impl StackEffect for Pick {
    fn run(&self, output: OutputFormat) {
        if let Ok(stack) = data::load(&self.stack) {
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

            data::save(&self.stack, stack).unwrap();

            Head {
                stack: self.stack.clone(),
                n: Some(seen.len()),
            }
            .run(output);
        }
    }
}

// ===== Move =====

/// Move the current item to a different stack.
pub struct Move {
    pub stack: String,
    pub dest_stack: String,
}

impl NamedEffect for Move {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "move",
            description: "Move current item to another stack",
            aliases: &[],
            input: EffectInput::RequiredSlurpy("destination"),
        }
    }
}

impl StackEffect for Move {
    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            if let Some(item) = items.pop() {
                data::save(&self.stack, items).unwrap();

                output.log(
                    vec!["action", "new-stack", "old-stack"],
                    vec![vec!["Move", &self.dest_stack, &self.stack]],
                );

                Push {
                    stack: self.dest_stack.clone(),
                    item,
                }
                .run(OutputFormat::Silent);
            }
        }
    }
}

// ===== MoveAll =====

/// Move all items to a different stack.
pub struct MoveAll {
    pub stack: String,
    pub dest_stack: String,
}

impl NamedEffect for MoveAll {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "move-all",
            description: "Move all items to another stack",
            aliases: &[],
            input: EffectInput::RequiredSlurpy("destination"),
        }
    }
}

impl StackEffect for MoveAll {
    fn run(&self, output: OutputFormat) {
        if let Ok(src_items) = data::load(&self.stack) {
            let count = src_items.len();

            if !src_items.is_empty() {
                let all_items = match data::load(&self.dest_stack) {
                    Ok(dest_items) => {
                        let mut all_items = dest_items;
                        for item in src_items {
                            all_items.push(item);
                        }
                        all_items
                    }
                    _ => src_items,
                };

                data::save(&self.dest_stack, all_items).unwrap();
                data::save(&self.stack, vec![]).unwrap();
            }

            output.log(
                vec!["action", "new-stack", "old-stack", "num-moved"],
                vec![vec![
                    "Move All",
                    &self.dest_stack,
                    &self.stack,
                    &count.to_string(),
                ]],
            );
        }
    }
}
