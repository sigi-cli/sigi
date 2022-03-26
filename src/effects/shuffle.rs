use super::{Head, Peek, StackEffect};
use crate::data;
use crate::data::Stack;
use crate::output::OutputFormat;

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

// ===== Swap =====

/// Swap the two most-recent items.
pub struct Swap {
    pub stack: String,
}

impl StackEffect for Swap {
    fn execute(&self, items: Stack) -> Stack {
        if items.len() < 2 {
            return vec![];
        }

        let mut items = items;
        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        items.push(a);
        items.push(b);

        items
    }

    fn after_execute(&self) -> Option<Box<dyn StackEffect>> {
        let head = Head {
            stack: self.stack,
            n: Some(2),
        };

        Some(Box::new(head))
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(stack) = data::load(&self.stack) {
            let result = self.execute(stack);

            data::save(&self.stack, result).unwrap();

            if let Some(effect) = self.after_execute() {
                effect.run(output);
            }
        }
    }
}

// ===== Rot =====

/// Rotate the three most-recent items.
pub struct Rot {
    pub stack: String,
}

impl StackEffect for Rot {
    fn execute(&self, items: Stack) -> Stack {
        if items.len() < 3 {
            let stack = self.stack;
            let swap = Swap { stack };
            return swap.execute(items);
        }

        let mut items = items;

        let a = items.pop().unwrap();
        let b = items.pop().unwrap();
        let c = items.pop().unwrap();

        items.push(a);
        items.push(c);
        items.push(b);

        items
    }

    fn after_execute(&self) -> Option<Box<dyn StackEffect>> {
        let head = Head {
            stack: self.stack,
            n: Some(3),
        };

        Some(Box::new(head))
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let result = self.execute(items);

            data::save(&self.stack, result).unwrap();

            if let Some(effect) = self.after_execute() {
                effect.run(output);
            }
        }
    }
}

// ===== Next =====

/// Make the next item the most-recent item.
/// The previously most-recent item is sent to the end of the stack.
pub struct Next {
    pub stack: String,
}

impl StackEffect for Next {
    fn execute(&self, items: Stack) -> Stack {
        if items.is_empty() {
            return items;
        }

        let mut items = items;
        let to_the_back = items.pop().unwrap();
        items.insert(0, to_the_back);

        items
    }

    fn after_execute(&self) -> Option<Box<dyn StackEffect>> {
        let peek = Peek {
            stack: self.stack.clone(),
        };

        Some(Box::new(peek))
    }

    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let result = self.execute(items);

            data::save(&self.stack, items).unwrap();

            if let Some(effect) = self.after_execute() {
                effect.run(output);
            }
        }
    }
}
