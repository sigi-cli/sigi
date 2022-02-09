use crate::data;
use crate::effects::{EffectInput, EffectNames, Head, NamedEffect, Peek, StackEffect};
use crate::output::SimpleTableData;

// TODO: Consider more shuffle words: https://docs.factorcode.org/content/article-shuffle-words.html

// ===== Swap =====

/// Swap the two most-recent items.
pub struct Swap {
    pub stack: String,
}

impl NamedEffect for Swap {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "swap",
            description: "Swap the two most-current items",
            aliases: &[],
            input: EffectInput::NoInput,
        }
    }
}

impl StackEffect for Swap {
    fn run(&self) -> Vec<SimpleTableData> {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;

            if items.len() > 1 {
                let a = items.pop().unwrap();
                let b = items.pop().unwrap();
                items.push(a);
                items.push(b);

                data::save(&self.stack, items).unwrap();
            }

            // Now show the first two items in their new order.
            return Head {
                stack: self.stack.clone(),
                n: Some(2),
            }
            .run();
        }

        vec![]
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

impl NamedEffect for Rot {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "rot",
            description: "Rotate the three most-current items",
            aliases: &["rotate"],
            input: EffectInput::NoInput,
        }
    }
}

impl StackEffect for Rot {
    fn run(&self) -> Vec<SimpleTableData> {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;

            if items.len() < 3 {
                return Swap {
                    stack: self.stack.clone(),
                }
                .run();
            }

            let a = items.pop().unwrap();
            let b = items.pop().unwrap();
            let c = items.pop().unwrap();

            items.push(a);
            items.push(c);
            items.push(b);

            data::save(&self.stack, items).unwrap();
            
            return Head {
                stack: self.stack.clone(),
                n: Some(3),
            }
            .run();
        }

        vec![]
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

impl NamedEffect for Next {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "next",
            description: "Cycle to the next item; the current item becomes last",
            aliases: &["later", "cycle", "bury"],
            input: EffectInput::NoInput,
        }
    }
}

impl StackEffect for Next {
    fn run(&self) -> Vec<SimpleTableData> {
        if let Ok(items) = data::load(&self.stack) {
            let mut items = items;

            if items.is_empty() {
                return vec![];
            }

            let to_the_back = items.pop().unwrap();
            items.insert(0, to_the_back);

            data::save(&self.stack, items).unwrap();
            
            return Peek {
                stack: self.stack.clone(),
            }
            .run();
        }

        vec![]
    }
}

impl From<&str> for Next {
    fn from(stack: &str) -> Next {
        Next {
            stack: stack.to_string(),
        }
    }
}
