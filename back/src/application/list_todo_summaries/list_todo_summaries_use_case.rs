use crate::application::todo_repository::TodoRepository;
use crate::domain::todo::Todo;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct TodoSummary {
    id: String,
    title: String,
    description_summary: String,
}

impl From<&Todo> for TodoSummary {
    fn from(todo: &Todo) -> Self {
        let title = todo.title.to_string();
        let short_description = &todo.description[..24];
        let description_summary = format!("{}...", short_description);
        TodoSummary {
            id: todo.id.clone(),
            title,
            description_summary,
        }
    }
}

pub struct ListTodoSummariesUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> ListTodoSummariesUseCase<T> {
    pub fn new(todo_repository: Arc<T>) -> Self {
        ListTodoSummariesUseCase { todo_repository }
    }

    pub fn act(&self) -> Vec<TodoSummary> {
        let todos = self.todo_repository.list();
        todos.iter().map(TodoSummary::from).collect()
    }
}
