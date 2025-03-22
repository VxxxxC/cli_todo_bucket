use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub input_type: InputType,
}

#[derive(Debug, Subcommand)]
pub enum InputType {
    /// Create, add new todo items
    Add(AddItem),
    /// Update the todo list
    Update(UpdateCommand),
    /// Check the current todo list
    Check,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[clap(subcommand)]
    pub command: UpdateSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UpdateSubcommand {
    /// Update todo item
    Update(UpdateItem),
    /// Delete todo item
    Delete(DeleteItem),
}

#[derive(Debug, Args)]
pub struct AddItem {
    /// update selected item
    pub item : String,
}

#[derive(Debug, Args)]
pub struct UpdateItem {
    /// update selected item
    pub item : String,
}

#[derive(Debug, Args)]
pub struct DeleteItem {
    /// update selected item
    pub item: String,
}
