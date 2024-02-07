//! Charcoal, a command line dictionary
//!
//! Charcoal uses youdao dict api and google speech. Inspired by wudao-dict.

pub mod app;
pub mod suggestion;
pub mod word;

pub use app::{
    builder::AppBuilder,
    cache::Cache,
    cli::{Cli, Commands},
    config::Config,
    App,
};
pub use suggestion::Suggestion;
pub use word::{
    frontend::SingleEntry, speech::Speech, Acquire, Answer, ExactQuery, PPrint, Question,
};
