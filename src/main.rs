use todo_list_iced_rust::todo_lib::{Todo, TodoCreate, TodoStatus, TodoUpdate, get_vec_from_file};
use iced::{Element, Sandbox, Settings, Theme, widget::{Column, Container, Button, TextInput, Row, Text, PickList, text::{Appearance, self}}, Alignment, Color};

fn main() -> iced::Result {

  TodoBox::run(Settings::default())
}

#[derive(Debug)]
pub struct TodoBox {
  pub todos: Vec<Todo>,
  pub todo_input_create: String,
  pub error_creation: String,
}

impl TodoBox {
  pub fn pourcent_of_completed(&self) -> f32 {
    let full_todos_number = self.todos.len();
    let todos_completed_number = self.todos.iter().filter(|todo| {
      match todo.status {
        TodoStatus::Completed => true,
        _ => false
      }
    }).count();

    if full_todos_number.eq(&0) || todos_completed_number.eq(&0) {
      0.0
    } else {
      (todos_completed_number as f32 / full_todos_number as f32) * 100.0
    }
    
  }
}

#[derive(Debug, Clone)]
pub enum TodoMessage{
  TodoCreateMessage,
  TodoUpdateMessage(String, String),
  TodoDeleteMessage(String),
  CreateTodoInputMessage(String),
}

struct TextError {
  color: iced::Color
}

// implemention default color red (hard one)
impl Default for TextError {
  fn default() -> Self {
      TextError {
          color: Color::from_rgb(1.0, 0.0, 0.0),
      }
  }
}

impl text::StyleSheet for TextError {
  type Style = ();

  fn appearance(&self, _style: Self::Style) -> Appearance {
      Appearance {
          color: Some(self.color),
          ..Default::default()
      }
  }
}


impl Sandbox for TodoBox {
    type Message = TodoMessage;
  
    fn new() -> TodoBox {
      TodoBox {
        todos: get_vec_from_file(),
        todo_input_create: String::new(),
        error_creation: String::new(),
      }
    }

    fn title(&self) -> String {
        String::from("Todo list")
    }

    fn update(&mut self, message: Self::Message) {
      match message {
        TodoMessage::CreateTodoInputMessage(new_value) => self.todo_input_create = new_value,
        TodoMessage::TodoCreateMessage => {
          match self.todo_input_create.clone() {
            value if value.len() == 0 => {
              self.error_creation = String::from("Todo task is require");
            },
            value if self.todos.iter().any(|v| v.task == value) => {
              self.error_creation = String::from("Task already exist");
            },
            _ => {
              let todo = Todo::new(
                TodoCreate { task: self.todo_input_create.clone() }
              );

              // can improve here this workflow
              self.todos.push(todo.clone());
              todo.push_arr_into_json(self.todos.clone());

              self.todo_input_create = String::new();
              self.error_creation = String::new();
            },
          }
        },
        TodoMessage::TodoUpdateMessage(value, id) => {
          match self.todos.iter().position(|todo| todo.id == id) {
            Some(u) => {
              // can improve here this workflow
              self.todos[u].update(TodoUpdate {status: get_status_from_string(value.as_str())});
              self.todos[u].clone().push_arr_into_json(self.todos.clone())
            },
            None => panic!("Status doesnt exist")
          };
        },
        TodoMessage::TodoDeleteMessage(id) => {
          // can improve here this workflow
          let new_list_todos: Vec<Todo> = self.todos.clone().drain(..).filter(|todo| todo.id != id).collect();
          self.todos[0].clone().push_arr_into_json(new_list_todos);
          self.todos.retain(|todo| {
            todo.id != id
          });
        },
      };
    }

    fn view(&self) -> Element<Self::Message> {
      let input_task = TextInput::new("Housework", &self.todo_input_create)
        .padding(10)
        .on_submit(TodoMessage::TodoCreateMessage)
        .on_input(TodoMessage::CreateTodoInputMessage);

      let button_submit_todo = Button::new("Create").on_press(TodoMessage::TodoCreateMessage).padding(10);


      let create_form = Column::new()
        .push(
          Text::new(
            format!("Todos completed : {:?} %", TodoBox::pourcent_of_completed(self))
          )
          .size(26) 
        )
        .spacing(15)
        .push(Text::new("Todo task :"))
        .spacing(5)
        .push(
        Row::new()
          .align_items(Alignment::Center)
          .push(input_task)
          .spacing(5)
          .push(button_submit_todo)
        )
        .spacing(5)
        .push(Text::new(self.error_creation.clone())
        .style(TextError::default().color)
      );
  
      let mut rows: Vec<Element<_>> = Vec::new();

      for todo in self.todos.iter() {
        rows.push(
          Row::new()
          .align_items(Alignment::Center)
          .push(Text::new(todo.task.clone()).size(28))
          .spacing(10)
          .push(
            PickList::new(
              vec![String::from("In progress"), String::from("Completed"), String::from("Pending")],
              Some(get_status_string(&todo.status)),
              move |selected_item| TodoMessage::TodoUpdateMessage(selected_item, todo.id.clone()),
            )
          )
          .spacing(10)
          .push(Button::new("Remove").padding(10).on_press(TodoMessage::TodoDeleteMessage(todo.id.clone())))
          .into()
        )
      }

      let list_todos = Column::with_children(rows).spacing(15);
      
      Container::new(
        Column::new()
          .push( create_form)
          .align_items(Alignment::Center)
          .spacing(40)
          .push(list_todos)
          .padding(20)
        )
        .into()
      
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn get_status_string(todo_status: &TodoStatus) -> String {
  match todo_status {
    TodoStatus::Completed => String::from("Completed"),
    TodoStatus::Pending => String::from("Pending"),
    TodoStatus::InProgress => String::from("In progress"),
  }
}

fn get_status_from_string(todo_status: &str) -> TodoStatus {
  match todo_status {
    "Completed"=> TodoStatus::Completed,
    "Pending"=> TodoStatus::Pending,
    "In progress" =>TodoStatus::InProgress,
    &_ => panic!("{} not in todo status", todo_status)
  }
}
