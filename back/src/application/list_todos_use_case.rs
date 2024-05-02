use std::sync::Arc;
use crate::application::todo_repository::TodoRepository;
use crate::domain::todo::Todo;

pub struct ListTodosUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> ListTodosUseCase<T> {

    pub fn new(todo_repository: Arc<T>) -> Self {
        ListTodosUseCase { todo_repository }
    }

    pub fn act(&self) -> Vec<Todo> {
        self.todo_repository.list()
    }
}
