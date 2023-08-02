use serde::{Serialize, Deserialize};

pub mod file_lib {
  use super::*;
  use std::{fs::File, io};

  #[derive(Serialize, Deserialize)]
  struct EmptyData {}

  pub fn get_file(path: String) -> Result<File, io::Error> {
    return File::open(&path);
  }
}


pub mod todo_lib {
  use super::*;
  use std::{io::{Read, Write}, fs::OpenOptions};
  use crate::file_lib::get_file;

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub enum TodoStatus {
    Completed,
    Pending,
    InProgress,
  }

  #[derive(Debug, Serialize, Deserialize, Clone)]
  pub struct Todo {
    pub id: String,
    pub task: String,
    pub status: TodoStatus,
  }

  pub struct TodoCreate {
    pub task: String
  }
  pub struct TodoUpdate {
    pub status: TodoStatus
  }

  impl Todo {
    pub fn new(todo_create: TodoCreate) -> Self {
      Todo { 
        id: get_random_id(),
        status: TodoStatus::Pending,
        task: todo_create.task,
      }    
    }

    pub fn update(&mut self, todo_update: TodoUpdate) {
      self.status = todo_update.status;
    }

    pub fn push_arr_into_json(self, todos: Vec<Todo>) {
      let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todos.json")
        .unwrap();
  
      let json_string = serde_json::to_string(&todos).unwrap();
  
      file.write_all(json_string.as_bytes()).unwrap();   
    }
  }
  fn get_random_id() -> String {
    uuid::Uuid::new_v4().to_string()
  }

  pub fn get_vec_from_file() -> Vec<Todo> {
    let mut content = String::new();
    let file = get_file("todos.json".to_string());

    if let Ok(mut file) = file {
      file.read_to_string(&mut content).expect("Cannot read file");
  
      let todos: Vec<Todo> = serde_json::from_str(&content).expect("Error while deserialize");
  
      return todos;
    } else {
      return vec![];
    }    
  }

}