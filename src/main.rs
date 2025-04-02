
mod cli;
mod todo;
mod reminder;

use std::time::SystemTime;
use actix_web::web::Json;
use chrono::Utc;
use clap::Parser;
use cli::{CliArgs, InputType, AddCommand, AddSubcommand, UpdateCommand, UpdateSubcommand, DeleteCommand, DeleteSubcommand};
use crate::todo::*;

#[actix_web::main]
async fn main() {
    let args: CliArgs = CliArgs::parse();
    // println!("{:?}", args.input_type);
    // println!("Data created at {}", Utc::now());

    // Check the args input here
    let mut input_args = std::env::args();
    input_args.next();
    let input = input_args.map(|x|x).collect::<Vec<_>>();
    // println!("args:{:?}",&input);

    match &input {
        input if input[0] == "add" => {
            if &input[1] == "todo" { add_todo_api(input[2].to_string()).await; }
            if &input[1] == "reminder" { println!("you are adding reminder!") }
        }
        input if input[0] == "update" => {
            if &input[1] == "todo" { println!("you are updating todo!") }
            if &input[1] == "reminder" { println!("you are updating reminder!") }
        }
        input if input[0] == "delete" => {
            if &input[1] == "todo" { println!("you are delete todo!") }
            if &input[1] == "reminder" { println!("you are delete reminder!") }
        }
        input if input[0] == "check" => println!("you are checking"),
        _ => {},
    }
}
