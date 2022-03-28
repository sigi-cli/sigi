use crate::data::Backend;
use crate::output::OutputFormat;

pub mod lifecycle;
pub use lifecycle::*;
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
    Push {
        stack: String,
        item: crate::data::Item,
    },
    Complete {
        stack: String,
    },
    Delete {
        stack: String,
    },
    DeleteAll {
        stack: String,
    },
    Pick {
        stack: String,
        indices: Vec<usize>,
    },
    Move {
        stack: String,
        dest: String,
    },
    MoveAll {
        stack: String,
        dest: String,
    },
    Swap {
        stack: String,
    },
    Rot {
        stack: String,
    },
    Next {
        stack: String,
    },
    Peek {
        stack: String,
    },
    ListAll {
        stack: String,
    },
    Head {
        stack: String,
        n: Option<usize>,
    },
    Tail {
        stack: String,
        n: Option<usize>,
    },
    Count {
        stack: String,
    },
    IsEmpty {
        stack: String,
    },
}

impl StackEffect {
    pub fn run(self, backend: &Backend, output: &OutputFormat) {
        match self {
            StackEffect::Push { stack, item } => Push { stack, item }.run(backend, output),
            StackEffect::Complete { stack } => Complete { stack }.run(backend, output),
            StackEffect::Delete { stack } => Delete { stack }.run(backend, output),
            StackEffect::DeleteAll { stack } => DeleteAll { stack }.run(backend, output),
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

// ===== Helper functions =====

fn stack_history_of(stack: &str) -> String {
    stack.to_string() + HISTORY_SUFFIX
}
