use std::sync::{Arc};
use uuid::Uuid;
use crate::application::todo_repository::TodoRepository;
use crate::domain::todo::Todo;

pub struct AddTodoUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> AddTodoUseCase<T> {

    pub fn new(todo_repository: Arc<T>) -> Self {
        AddTodoUseCase { todo_repository }
    }

    pub fn act(&self, title: &str) -> Todo {
        let id = Uuid::new_v4();
        let todo = Todo::new(&id.to_string(), title);
        self.todo_repository.add(todo)
    }
}
