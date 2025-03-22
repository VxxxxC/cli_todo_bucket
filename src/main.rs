mod cli;
mod todo;
mod reminder;

use clap::Parser;
use cli::CliArgs;

fn main() {
    let args: CliArgs = CliArgs::parse();
}
