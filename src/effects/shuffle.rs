use crate::data::Backend;
use crate::effects::{Head, Peek, StackAction};
use crate::output::OutputFormat;

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

// ===== Swap =====

/// Swap the two most-recent items.
pub struct Swap {
    pub stack: String,
}

impl StackAction for Swap {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        let stack = self.stack;
        if let Ok(items) = backend.load(&stack) {
            let mut items = items;

            if items.len() < 2 {
                return;
            }

            let a = items.pop().unwrap();
            let b = items.pop().unwrap();
            items.push(a);
            items.push(b);

            backend.save(&stack, items).unwrap();

            // Now show the first two items in their new order.
            let n = Some(2);
            let head = Head { stack, n };
            head.run(backend, output);
        }
    }
}

// ===== Rot =====

/// Rotate the three most-recent items.
pub struct Rot {
    pub stack: String,
}

impl StackAction for Rot {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        let stack = self.stack;
        if let Ok(items) = backend.load(&stack) {
            let mut items = items;

            if items.len() < 3 {
                let swap = Swap { stack };
                swap.run(backend, output);
                return;
            }

            let a = items.pop().unwrap();
            let b = items.pop().unwrap();
            let c = items.pop().unwrap();

            items.push(a);
            items.push(c);
            items.push(b);

            backend.save(&stack, items).unwrap();

            let n = Some(3);
            let head = Head { stack, n };
            head.run(backend, output);
        }
    }
}

// ===== Next =====

/// Make the next item the most-recent item.
/// The previously most-recent item is sent to the end of the stack.
pub struct Next {
    pub stack: String,
}

impl StackAction for Next {
    fn run(self, backend: &Backend, output: &OutputFormat) {
        let stack = self.stack;
        if let Ok(items) = backend.load(&stack) {
            let mut items = items;
            if items.is_empty() {
                return;
            }
            let to_the_back = items.pop().unwrap();
            items.insert(0, to_the_back);

            backend.save(&stack, items).unwrap();

            let peek = Peek { stack };

            peek.run(backend, output);
        }
    }
}
