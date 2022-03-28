use crate::data::{Backend, Item};
use crate::output::OutputFormat;

pub mod views;
pub use views::*;
pub mod shuffle;
pub use shuffle::*;
pub mod housekeeping;
pub use housekeeping::*;

const HISTORY_SUFFIX: &str = "_history";

// TODO: Unify StackAction and StackEffect
pub trait StackAction {
    fn run(self, backend: &Backend, output: &OutputFormat);
}

pub enum StackEffect {
    Push { stack: String, content: String },
    Complete { stack: String },
    Delete { stack: String },
    DeleteAll { stack: String },
    Pick { stack: String, indices: Vec<usize> },
    Move { stack: String, dest: String },
    MoveAll { stack: String, dest: String },
    Swap { stack: String },
    Rot { stack: String },
    Next { stack: String },
    Peek { stack: String },
    ListAll { stack: String },
    Head { stack: String, n: Option<usize> },
    Tail { stack: String, n: Option<usize> },
    Count { stack: String },
    IsEmpty { stack: String },
}

impl StackEffect {
    pub fn run(self, backend: &Backend, output: &OutputFormat) {
        match self {
            StackEffect::Push { stack, content } => {
                let item = Item::new(&content);
                push_item(stack, item, backend, output)
            },
            StackEffect::Complete { stack } => complete_latest_item(stack, backend, output),
            StackEffect::Delete { stack } => delete_latest_item(stack, backend, output),
            StackEffect::DeleteAll { stack } => delete_all_items(stack, backend, output),
            StackEffect::Pick { stack, indices } => Pick { stack, indices }.run(backend, output),
            StackEffect::Move { stack, dest } => Move { stack, dest }.run(backend, output),
            StackEffect::MoveAll { stack, dest } => MoveAll { stack, dest }.run(backend, output),
            StackEffect::Swap { stack } => Swap { stack }.run(backend, output),
            StackEffect::Rot { stack } => Rot { stack }.run(backend, output),
            StackEffect::Next { stack } => Next { stack }.run(backend, output),
            StackEffect::Peek { stack } => Peek { stack }.run(backend, output),
            StackEffect::ListAll { stack } => ListAll { stack }.run(backend, output),
            StackEffect::Head { stack, n } => Head { stack, n }.run(backend, output),
            StackEffect::Tail { stack, n } => Tail { stack, n }.run(backend, output),
            StackEffect::Count { stack } => Count { stack }.run(backend, output),
            StackEffect::IsEmpty { stack } => IsEmpty { stack }.run(backend, output),
        }
    }
}

pub fn push_item(stack: String, item: Item, backend: &Backend, output: &OutputFormat) {
    let contents = item.contents.clone();

    let items = if let Ok(items) = backend.load(&stack) {
        let mut items = items;
        items.push(item);
        items
    } else {
        vec![item]
    };

    backend.save(&stack, items).unwrap();

    output.log(vec!["action", "item"], vec![vec!["Created", &contents]]);
}

fn complete_latest_item(stack: String, backend: &Backend, output: &OutputFormat) {
    if let Ok(items) = backend.load(&stack) {
        let mut items = items;
        if let Some(item) = items.pop() {
            let mut item = item;
            item.mark_completed();

            // Push the now-marked-completed item to history stack.
            push_item(
                stack_history_of(&stack),
                item.clone(),
                backend,
                &OutputFormat::Silent,
            );

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

fn delete_latest_item(stack: String, backend: &Backend, output: &OutputFormat) {
    if let Ok(items) = backend.load(&stack) {
        let mut items = items;
        if let Some(item) = items.pop() {
            let mut item = item;
            item.mark_deleted();

            // Push the now-marked-deleted item to history stack.
            push_item(
                stack_history_of(&stack),
                item.clone(),
                backend,
                &OutputFormat::Silent,
            );

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

fn delete_all_items(stack: String, backend: &Backend, output: &OutputFormat) {
    if let Ok(items) = backend.load(&stack) {
        let mut items = items;
        items.iter_mut().for_each(|item| item.mark_deleted());
        let n_deleted = items.len();

        // Push the now-marked-deleted items to history stack.
        let history_stack = &stack_history_of(&stack);
        let mut history = backend.load(history_stack).unwrap_or_default();
        history.append(&mut items);
        backend.save(history_stack, history).unwrap();

        // Save the original stack as empty now.
        backend.save(&stack, vec![]).unwrap();

        output.log(
            vec!["action", "item"],
            vec![vec!["Deleted", &format!("{} items", n_deleted)]],
        );
    }
}

// ===== Helper functions =====

fn stack_history_of(stack: &str) -> String {
    stack.to_string() + HISTORY_SUFFIX
}
