use std::io::Error;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct Todo {
    pub item: String,
}

impl Todo {
  pub fn add_todo(todo: Todo) -> Result<Todo, Error>{
    println!("todo item : {}", todo.item);
    Ok(todo)
  }
}

// FIXME:
pub async fn add_todo_api(url: String, item: String) {

    let client = reqwest::Client::new();
    let post = url + "/post";

    let input = Todo {
        item
    };
    let todo = Todo::add_todo(input);
    match todo {
        Ok(todo) => {
            let add_to_list: Todo = Todo {
                item: todo.item
            };

            let res = client.post(post).json(&add_to_list).send().await;
            println!("res : {:?}", res);

        }
        Err(_) => println!("Failed to add todo item"),
    }
}
