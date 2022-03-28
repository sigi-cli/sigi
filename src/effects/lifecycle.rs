use crate::data::Backend;
use crate::effects::{stack_history_of, Peek, StackAction};
use crate::output::OutputFormat;

// ===== Create (Push) =====

/// Add a new item.
pub struct Push {
    pub stack: String,
    pub item: crate::data::Item,
}

impl StackAction for Push {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        let new_items = if let Ok(items) = backend.load(&self.stack) {
            let mut items = items;
            items.push(self.item.clone());
            items
        } else {
            vec![self.item.clone()]
        };
        backend.save(&self.stack, new_items).unwrap();
        output.log(
            vec!["action", "item"],
            vec![vec!["Created", &self.item.contents]],
        );
    }
}

// ===== Complete (Pop) =====

/// Complete (successfully) the most-recent item.
pub struct Complete {
    pub stack: String,
}

impl StackAction for Complete {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        let stack = self.stack.clone();
        if let Ok(items) = backend.load(&self.stack) {
            let mut items = items;
            if let Some(item) = items.pop() {
                let mut item = item;
                item.mark_completed();

                // Push the now-marked-completed item to history stack.
                let push = Push {
                    stack: stack_history_of(&stack),
                    item: item.clone(),
                };

                push.run(backend, &OutputFormat::Silent);

                // Save the original stack without that item.
                backend.save(&stack, items).unwrap();

                output.log(
                    vec!["action", "item"],
                    vec![vec!["Completed", &item.contents]],
                );
            }
        }

        // Peek the current stack only for human output.
        if let OutputFormat::Human(_) = output {
            let peek = Peek { stack };
            peek.run(backend, output);
        }
    }
}

// ===== Delete (Pop) =====

/// Delete the most-recent item.
pub struct Delete {
    pub stack: String,
}

impl StackAction for Delete {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        let stack = self.stack;
        if let Ok(items) = backend.load(&stack) {
            let mut items = items;
            if let Some(item) = items.pop() {
                let mut item = item;
                item.mark_deleted();

                // Push the now-marked-deleted item to history stack.
                let push = Push {
                    stack: stack_history_of(&stack),
                    item: item.clone(),
                };

                push.run(backend, &OutputFormat::Silent);

                // Save the original stack without that item.
                backend.save(&stack, items).unwrap();

                output.log(
                    vec!["action", "item"],
                    vec![vec!["Deleted", &item.contents]],
                );
            }
        }

        // Peek the current stack only for human output.
        if let OutputFormat::Human(_) = output {
            let peek = Peek { stack };
            peek.run(backend, output);
        }
    }
}

// ===== DeleteAll (Pop all) =====

/// Delete all items.
///
/// Note: Deleted items are moved to a stack with the same name and the suffix `_history`.
pub struct DeleteAll {
    pub stack: String,
}

impl StackAction for DeleteAll {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        if let Ok(items) = backend.load(&self.stack) {
            let mut items = items;
            items.iter_mut().for_each(|item| item.mark_deleted());
            let n_deleted = items.len();

            // Push the now-marked-deleted items to history stack.
            let history_stack = &stack_history_of(&self.stack);
            let mut history = backend.load(history_stack).unwrap_or_default();
            history.append(&mut items);
            backend.save(history_stack, history).unwrap();

            // Save the original stack as empty now.
            backend.save(&self.stack, vec![]).unwrap();

            output.log(
                vec!["action", "item"],
                vec![vec!["Deleted", &format!("{} items", n_deleted)]],
            );
        }
    }
}
