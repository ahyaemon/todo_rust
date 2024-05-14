use crate::application::todo_repository::TodoRepository;
use std::sync::Arc;

pub struct DeleteTodoUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> DeleteTodoUseCase<T> {
    pub fn new(todo_repository: Arc<T>) -> Self {
        DeleteTodoUseCase { todo_repository }
    }

    pub fn act(&self, id: &str) {
        self.todo_repository.delete(id)
    }
}
