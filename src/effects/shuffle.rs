use crate::data;
use crate::effects::{Head, Peek, StackEffect};
use crate::output::OutputFormat;

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

// ===== Swap =====

/// Swap the two most-recent items.
pub struct Swap {
    pub stack: String,
}

impl StackEffect for Swap {
    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            if items.len() < 2 {
                return;
            }
            let a = items.pop().unwrap();
            let b = items.pop().unwrap();
            items.push(a);
            items.push(b);

            data::save(&self.stack, items).unwrap();

            // Now show the first two items in their new order.
            Head {
                stack: self.stack.clone(),
                n: Some(2),
            }
            .run(output);
        }
    }
}

impl From<&str> for Swap {
    fn from(stack: &str) -> Swap {
        Swap {
            stack: stack.to_string(),
        }
    }
}

// ===== Rot =====

/// Rotate the three most-recent items.
pub struct Rot {
    pub stack: String,
}

impl StackEffect for Rot {
    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;

            if items.len() < 3 {
                Swap {
                    stack: self.stack.clone(),
                }
                .run(output);
                return;
            }

            let a = items.pop().unwrap();
            let b = items.pop().unwrap();
            let c = items.pop().unwrap();

            items.push(a);
            items.push(c);
            items.push(b);

            data::save(&self.stack, items).unwrap();
            Head {
                stack: self.stack.clone(),
                n: Some(3),
            }
            .run(output);
        }
    }
}

impl From<&str> for Rot {
    fn from(stack: &str) -> Rot {
        Rot {
            stack: stack.to_string(),
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
    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;
            if items.is_empty() {
                return;
            }
            let to_the_back = items.pop().unwrap();
            items.insert(0, to_the_back);

            data::save(&self.stack, items).unwrap();
            Peek {
                stack: self.stack.clone(),
            }
            .run(output);
        }
    }
}

impl From<&str> for Next {
    fn from(stack: &str) -> Next {
        Next {
            stack: stack.to_string(),
        }
    }
}
