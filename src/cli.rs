use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    /// First Args
    pub first_args: String,
    /// Second Args
    pub second_args: String,
    /// Third Args
    pub third_args: String,
}
