use crate::data;
use crate::effects::{EffectInput, EffectNames, Head, Peek, StackEffect};
use crate::output::OutputFormat;

// ===== Swap =====

pub struct Swap {
    pub stack: String,
}

impl StackEffect for Swap {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "swap",
            description: "Swap the two most-current items",
            aliases: &[],
            input: EffectInput::NoInput,
        }
    }

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

// ===== Rot =====

pub struct Rot {
    pub stack: String,
}

impl StackEffect for Rot {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "rot",
            description: "Rotate the three most-current items",
            aliases: &["rotate"],
            input: EffectInput::NoInput,
        }
    }

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

// ===== Next =====

pub struct Next {
    pub stack: String,
}

impl StackEffect for Next {
    fn names<'a>() -> EffectNames<'a> {
        EffectNames {
            name: "next",
            description: "Cycle to the next item; the current item becomes last",
            aliases: &["later", "cycle", "bury"],
            input: EffectInput::NoInput,
        }
    }

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
