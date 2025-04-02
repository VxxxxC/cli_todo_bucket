
mod cli;
mod todo;
mod reminder;

use dotenv::dotenv;
use std::env;
use clap::Parser;
use cli::CliArgs;
use crate::todo::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let env = dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let uri = format!("http://{}:{}", host, port);

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
            if &input[1] == "todo" { add_todo_api(uri, input[2].to_string()).await; }
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

    Ok(())
}
