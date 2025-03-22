use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args)]
pub struct Reminder {
    pub item: String,
    pub time: usize,
}
