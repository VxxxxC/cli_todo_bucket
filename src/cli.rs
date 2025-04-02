use clap::{Args, Parser, Subcommand};
use crate::todo::Todo;
use crate::reminder::Reminder;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    pub url: String,
    
    #[clap(subcommand)]
    pub input_type: InputType,
}

#[derive(Debug, Subcommand)]
pub enum InputType {
    #[clap(about="Add new item")]
    Add(AddCommand),
    #[clap(about="Update the list")]
    Update(UpdateCommand),
    #[clap(about="Delete item")]
    Delete(DeleteCommand),
    #[clap(about="Check current list")]
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
    #[clap(about="Add todo item")]
    Todo(Todo),
    #[clap(about="Add reminder item")]
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
    #[clap(about="Update todo item")]
    Todo(Todo),
    #[clap(about="Update reminder item")]
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
    #[clap(about="Delete todo item")]
    Todo(Todo),
    #[clap(about="Delete reminder item")]
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
