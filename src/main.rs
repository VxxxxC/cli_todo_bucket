mod cli;

use clap::Parser;
use cli::CliArgs;

fn main() {
    let args: CliArgs = CliArgs::parse();
}
