use clap::{Args, Parser, Subcommand};

#[derive(Debug, Args)]
pub struct Todo {
    pub item: String,
}