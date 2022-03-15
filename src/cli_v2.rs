use clap::{ArgEnum, Parser, Subcommand};
use std::str::FromStr;
use std::{error, fmt};

/// The current version of the CLI. (As defined in Cargo.toml)
pub const SIGI_VERSION: &str = std::env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(name = "sigi", version = SIGI_VERSION)]
/// An organizing tool for terminal lovers who hate organizing
struct Cli {
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
    format: Option<Format>,

    #[clap(short='t', long, visible_aliases = &["topic", "about", "namespace"])]
    /// Manage items in a specific stack
    stack: Option<String>,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Move the current item to "<STACK>_history" and mark as completed
    #[clap(visible_aliases = &["done", "finish", "fulfill"])]
    Complete {},

    /// Print the total number of items in the stack
    #[clap(visible_aliases = &["size", "length"])]
    Count {},

    /// Move the current item to "<STACK>_history" and mark as deleted
    #[clap(visible_aliases = &["pop", "remove", "cancel", "drop"])]
    Delete {},

    /// Move all items to "<STACK>_history" and mark as deleted
    #[clap(visible_aliases = &["purge", "pop-all", "remove-all", "cancel-all", "drop-all"])]
    DeleteAll {},

    /// List the first N items
    #[clap(visible_aliases = &["top", "first"])]
    Head { n: Option<usize> },

    /// Print "true" if stack has zero items, or print "false" (and exit with a
    /// nonzero exit code) if the stack does have items
    #[clap(visible_aliases = &["empty"])]
    IsEmpty {},

    /// List all items
    #[clap(visible_aliases = &["ls", "snoop", "show", "all"])]
    List {},

    /// Move current item to another stack
    #[clap(arg_required_else_help = true)]
    Move { destination: String },

    /// Move all items to another stack
    #[clap(arg_required_else_help = true)]
    MoveAll { destination: String },

    /// Cycle to the next item; the current item becomes last
    #[clap(visible_aliases = &["later", "cycle", "bury"])]
    Next {},

    /// Show the first item. This is the default behavior when no command is given
    #[clap(visible_aliases = &["show"])]
    Peek {},

    /// Move items to the top of stack by their number
    Pick { ns: Vec<usize> },

    /// Create a new item
    #[clap(visible_aliases = &["create", "add", "do", "start", "new"])]
    Push {},

    /// Rotate the three most-current items
    #[clap(visible_aliases = &["rotate"])]
    Rot {},

    /// Swap the two most-current items
    Swap {},

    /// List the last N items
    #[clap(visible_aliases = &["bottom", "last"])]
    Tail { n: Option<usize> },
}

#[derive(ArgEnum, Clone)]
#[clap(arg_enum)]
enum Format {
    Csv,
    Json,
    JsonCompact,
    Tsv,
}

#[derive(Debug)]
struct UnknownFormat {
    format: String,
}

impl fmt::Display for UnknownFormat {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(out, "Unknown format: {}", self.format)
    }
}

impl error::Error for UnknownFormat {}

impl FromStr for Format {
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

pub fn run() {
    let args = Cli::parse();

    match &args.command {
        _ => {
            todo!("ALL THE THINGS")
        }
    }
}
