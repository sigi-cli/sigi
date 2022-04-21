use crate::data::Backend;
use crate::effects::StackEffect;
use crate::output::{NoiseLevel, OutputFormat};
use clap::{ArgEnum, Args, Parser, Subcommand};
use std::str::FromStr;
use std::{error, fmt};

mod interact;
use interact::*;

/// The current version of the CLI. (As defined in Cargo.toml)
pub const SIGI_VERSION: &str = std::env!("CARGO_PKG_VERSION");

const DEFAULT_STACK_NAME: &str = "sigi";
const DEFAULT_FORMAT: OutputFormat = OutputFormat::Human(NoiseLevel::Normal);
const DEFAULT_BACKEND: Backend = Backend::HomeDir;
const DEFAULT_SHORT_LIST_LIMIT: usize = 10;

// === Glossary ===
const COMPLETE_TERMS: [&str; 4] = ["complete", "done", "finish", "fulfill"];
const COUNT_TERMS: [&str; 3] = ["count", "size", "length"];
const DELETE_TERMS: [&str; 5] = ["delete", "pop", "remove", "cancel", "drop"];
const DELETE_ALL_TERMS: [&str; 6] = [
    "delete-all",
    "purge",
    "pop-all",
    "remove-all",
    "cancel-all",
    "drop-all",
];
const HEAD_TERMS: [&str; 3] = ["head", "top", "first"];
const IS_EMPTY_TERMS: [&str; 2] = ["is-empty", "empty"];
const LIST_TERMS: [&str; 4] = ["ls", "snoop", "show", "all"];
const LIST_STACKS_TERMS: [&str; 2] = ["list-stacks", "stacks"];
const MOVE_TERM: &str = "move";
const MOVE_ALL_TERM: &str = "move-all";
const NEXT_TERMS: [&str; 4] = ["next", "later", "cycle", "bury"];
const PEEK_TERMS: [&str; 2] = ["peek", "show"];
const PICK_TERM: &str = "pick";
const PUSH_TERMS: [&str; 6] = ["push", "create", "add", "do", "start", "new"];
const ROT_TERMS: [&str; 2] = ["rot", "rotate"];
const SWAP_TERM: &str = "swap";
const TAIL_TERMS: [&str; 3] = ["tail", "bottom", "last"];
// === /glossary ===

pub fn run() {
    let args = Cli::parse();

    let stack = args.stack.unwrap_or_else(|| DEFAULT_STACK_NAME.into());

    match args.mode {
        None => {
            let output = args.fc.into_output_format().unwrap_or(DEFAULT_FORMAT);
            let peek = StackEffect::Peek { stack };
            peek.run(&DEFAULT_BACKEND, &output);
        }
        Some(Mode::Command(command)) => {
            let (effect, effect_fc) = command.into_effect_and_fc(stack);
            let output = args.fc.into_fallback_for(effect_fc);
            effect.run(&DEFAULT_BACKEND, &output);
        }
        Some(Mode::Interactive { fc }) => {
            let output = args.fc.into_fallback_for(fc);
            interact(stack, output);
        }
    };
}

#[derive(Parser)]
#[clap(name = "sigi", version = SIGI_VERSION, after_help = INTERACT_INSTRUCTIONS, after_long_help = INTERACT_LONG_INSTRUCTIONS)]
/// An organizing tool for terminal lovers who hate organizing
struct Cli {
    #[clap(flatten)]
    fc: FormatConfig,

    #[clap(short='t', long, visible_aliases = &["topic", "about", "namespace"])]
    /// Manage items in a specific stack
    stack: Option<String>,

    #[clap(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    /// Run in an interactive mode
    #[clap(visible_alias = "i")]
    Interactive {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    #[clap(flatten)]
    Command(Command),
}

#[derive(Subcommand)]
enum Command {
    /// Move the current item to "<STACK>_history" and mark as completed
    #[clap(visible_aliases = &COMPLETE_TERMS[1..])]
    Complete {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Print the total number of items in the stack
    #[clap(visible_aliases = &COUNT_TERMS[1..])]
    Count {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Move the current item to "<STACK>_history" and mark as deleted
    #[clap(visible_aliases = &DELETE_TERMS[1..])]
    Delete {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Move all items to "<STACK>_history" and mark as deleted
    #[clap(visible_aliases = &DELETE_ALL_TERMS[1..])]
    DeleteAll {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// List the first N items (default is 10)
    #[clap(visible_aliases = &HEAD_TERMS[1..])]
    Head {
        /// The number of items to display
        n: Option<usize>,

        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Print "true" if stack has zero items, or print "false" (and exit with a
    /// nonzero exit code) if the stack does have items
    #[clap(visible_aliases = &IS_EMPTY_TERMS[1..])]
    IsEmpty {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// List all items
    #[clap(visible_aliases = &LIST_TERMS[1..])]
    List {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// List all stacks
    #[clap(visible_aliases = &LIST_STACKS_TERMS[1..])]
    ListStacks {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Move current item to another stack
    #[clap(arg_required_else_help = true)]
    Move {
        #[clap(name = "destination")]
        /// The stack that will get the source stack's current item
        dest: String,

        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Move all items to another stack
    #[clap(arg_required_else_help = true)]
    MoveAll {
        #[clap(name = "destination")]
        /// The stack that will get all the source stack's items
        dest: String,

        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Cycle to the next item; the current item becomes last
    #[clap(visible_aliases = &NEXT_TERMS[1..])]
    Next {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Show the first item. This is the default behavior when no command is given
    #[clap(visible_aliases = &PEEK_TERMS[1..])]
    Peek {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Move items to the top of stack by their number
    Pick {
        ns: Vec<usize>,

        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Create a new item
    #[clap(visible_aliases = &PUSH_TERMS[1..])]
    Push {
        // The content to add as an item. Multiple arguments will be interpreted as a single string
        content: Vec<String>,

        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Rotate the three most-current items
    #[clap(visible_aliases = &ROT_TERMS[1..])]
    Rot {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// Swap the two most-current items
    Swap {
        #[clap(flatten)]
        fc: FormatConfig,
    },

    /// List the last N items (default is 10)
    #[clap(visible_aliases = &TAIL_TERMS[1..])]
    Tail {
        /// The number of items to display
        n: Option<usize>,

        #[clap(flatten)]
        fc: FormatConfig,
    },
}

impl Command {
    fn into_effect_and_fc(self, stack: String) -> (StackEffect, FormatConfig) {
        match self {
            Command::Complete { fc } => (StackEffect::Complete { stack }, fc),
            Command::Count { fc } => (StackEffect::Count { stack }, fc),
            Command::Delete { fc } => (StackEffect::Delete { stack }, fc),
            Command::DeleteAll { fc } => (StackEffect::DeleteAll { stack }, fc),
            Command::Head { n, fc } => {
                let n = n.unwrap_or(DEFAULT_SHORT_LIST_LIMIT);
                (StackEffect::Head { n, stack }, fc)
            }
            Command::IsEmpty { fc } => (StackEffect::IsEmpty { stack }, fc),
            Command::List { fc } => (StackEffect::ListAll { stack }, fc),
            Command::ListStacks { fc } => (StackEffect::ListStacks, fc),
            Command::Move { dest, fc } => (StackEffect::Move { stack, dest }, fc),
            Command::MoveAll { dest, fc } => (StackEffect::MoveAll { stack, dest }, fc),
            Command::Next { fc } => (StackEffect::Next { stack }, fc),
            Command::Peek { fc } => (StackEffect::Peek { stack }, fc),
            Command::Pick { ns, fc } => (StackEffect::Pick { stack, indices: ns }, fc),
            Command::Push { content, fc } => {
                let content = content.join(" ");
                (StackEffect::Push { stack, content }, fc)
            }
            Command::Rot { fc } => (StackEffect::Rot { stack }, fc),
            Command::Swap { fc } => (StackEffect::Swap { stack }, fc),
            Command::Tail { n, fc } => {
                let n = n.unwrap_or(DEFAULT_SHORT_LIST_LIMIT);
                (StackEffect::Tail { n, stack }, fc)
            }
        }
    }
}

#[derive(Args)]
struct FormatConfig {
    #[clap(short, long)]
    /// Omit any leading labels or symbols. Recommended for use in shell scripts
    quiet: bool,

    #[clap(short, long)]
    /// Omit any output at all
    silent: bool,

    #[clap(short, long, visible_alias = "noisy")]
    /// Print more information, like when an item was created
    verbose: bool,

    #[clap(short, long)]
    /// Use a programmatic format. Options include [csv, json, json-compact, tsv]. Not compatible with quiet/silent/verbose.
    format: Option<ProgrammaticFormat>,
}

impl FormatConfig {
    fn into_output_format(self) -> Option<OutputFormat> {
        let FormatConfig {
            verbose,
            silent,
            quiet,
            format,
        } = self;
        format
            .map(|format| match format {
                ProgrammaticFormat::Csv => OutputFormat::Csv,
                ProgrammaticFormat::Json => OutputFormat::Json,
                ProgrammaticFormat::JsonCompact => OutputFormat::JsonCompact,
                ProgrammaticFormat::Tsv => OutputFormat::Tsv,
            })
            .or_else(|| {
                if verbose {
                    Some(OutputFormat::Human(NoiseLevel::Verbose))
                } else if silent {
                    Some(OutputFormat::Silent)
                } else if quiet {
                    Some(OutputFormat::Human(NoiseLevel::Quiet))
                } else {
                    None
                }
            })
    }

    fn into_fallback_for(self, fc: FormatConfig) -> OutputFormat {
        fc.into_output_format()
            .or_else(|| self.into_output_format())
            .unwrap_or(DEFAULT_FORMAT)
    }
}

#[derive(ArgEnum, Clone)]
#[clap(arg_enum)]
enum ProgrammaticFormat {
    Csv,
    Json,
    JsonCompact,
    Tsv,
}

impl FromStr for ProgrammaticFormat {
    type Err = UnknownFormat;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        let format = format.to_ascii_lowercase();
        match format.as_str() {
            "csv" => Ok(Self::Csv),
            "json" => Ok(Self::Json),
            "json-compact" => Ok(Self::JsonCompact),
            "tsv" => Ok(Self::Tsv),
            _ => Err(UnknownFormat { format }),
        }
    }
}

#[derive(Debug)]
struct UnknownFormat {
    format: String,
}

impl error::Error for UnknownFormat {}

impl fmt::Display for UnknownFormat {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(out, "Unknown format: {}", self.format)
    }
}
