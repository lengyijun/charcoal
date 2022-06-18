use clap::{Args, Parser, Subcommand, ValueEnum};

/// A command line dictionary
#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn new() -> Command {
        Self::parse().command
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Query words from online or offline
    #[clap(aliases = &["q", "search", "s"])]
    Query(QueryArgs),
    /// Edit the configuration file
    #[clap(aliases = &["e", "config"])]
    Edit(EditArgs),
    /// Clean cache
    #[clap(aliases = &["c"])]
    Clean,
}

#[derive(Args, Debug)]
pub struct QueryArgs {
    /// The word to be queried
    #[clap(value_parser)]
    pub query: String,
    /// Whether to speak aloud
    #[clap(value_parser, short, long)]
    pub speak: Option<Toggle>,
    /// Whether to be concise
    #[clap(value_parser, short, long)]
    pub concise: Option<Toggle>,
}

#[derive(Args, Debug)]
pub struct EditArgs {
    /// A fresh start
    #[clap(value_parser, long)]
    pub reset: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Toggle {
    /// Yes
    Y,
    /// No
    N,
    /// Toggle
    T,
}

impl Toggle {
    pub fn twitch(&self, b: &mut bool) {
        match self {
            Toggle::Y => *b = true,
            Toggle::N => *b = false,
            Toggle::T => *b = !*b,
        }
    }
    pub fn counter_twitch(&self, b: &mut bool) {
        match self {
            Toggle::Y => *b = false,
            Toggle::N => *b = true,
            Toggle::T => *b = !*b,
        }
    }
}
