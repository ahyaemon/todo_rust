use std::sync::Arc;
use crate::application::todo_repository::TodoRepository;
use crate::domain::todo::Todo;

pub struct GetTodoUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> GetTodoUseCase<T> {

    pub fn new(todo_repository: Arc<T>) -> Self {
        GetTodoUseCase { todo_repository }
    }

    pub fn act(&self, id: &str) -> Todo {
        self.todo_repository.get(id)
    }
}
