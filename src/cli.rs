use clap::{Args, Parser, Subcommand};
use crate::todo::Todo;
use crate::reminder::Reminder;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub input_type: InputType,
}

#[derive(Debug, Subcommand)]
pub enum InputType {
    /// Add new item
    Add(AddCommand),
    /// Update the list
    Update(UpdateCommand),
    /// Check the current list
    Check,
}

// ADD
#[derive(Debug, Args)]
pub struct AddCommand {
    #[clap(subcommand)]
    pub add: AddSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AddSubcommand {
    /// Add todo item
    Todo(Todo),
    /// Add reminder item
    Reminder(Reminder),
}

// UPDATE
#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[clap(subcommand)]
    pub update: UpdateSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UpdateSubcommand {
    /// Update todo item
    Todo(Todo),
    /// Update reminder item
    Reminder(Reminder),
}

// DELETE
#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[clap(subcommand)]
    pub delete: DeleteSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DeleteSubcommand {
    /// Delete todo item
    Todo(Todo),
    /// Delete reminder item
    Reminder(Reminder),
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
