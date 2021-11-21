use crate::output::{NoiseLevel, OutputFormat};
use crate::{data, data::Item, data::Stack};

pub trait StackEffect {
    fn names() -> EffectNames;
    fn run(&self, output: OutputFormat);
}

pub struct EffectNames {
    pub name: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

const PEEK_ALIASES: [&str; 1] = ["show"];
const PEEK_NAMES: EffectNames = EffectNames {
    name: "peek",
    description: "Show the current item",
    aliases: &PEEK_ALIASES,
};

pub struct Peek {
    pub stack: String,
}

impl StackEffect for Peek {
    fn names() -> EffectNames {
        PEEK_NAMES
    }
    fn run(&self, output: OutputFormat) {
        if let Ok(items) = data::load(&self.stack) {
            if !items.is_empty() {
                output.log(
                    vec!["num", "item"],
                    vec![vec!["Now:", &items.last().unwrap().name]],
                );
            }
        }
    }
}