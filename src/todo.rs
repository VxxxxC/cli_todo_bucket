use std::io::Error;

use actix_web::{HttpResponse, post, web::Json};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct Todo {
    pub item: String,
}

impl Todo {
  pub fn add_todo(todo: Todo) -> Result<Todo, Error>{
    Ok(todo)
  }
}

#[post("/api/add_todo")] // FIXME:
pub async fn add_todo_api(item: String) -> HttpResponse {
    let input = Todo {
        item
    };
    let todo = Todo::add_todo(input);
    match todo {
        Ok(todo) => {
            let add_to_list: Todo = Todo {
                item: todo.item
            };
            HttpResponse::Ok().body(format!("Added todo: {}", add_to_list.item))
        }
        Err(_) => HttpResponse::BadRequest().body("Failed to add todo item"),
    }
}