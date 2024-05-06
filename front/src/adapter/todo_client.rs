use serde::Deserialize;
use reqwest::Result;
use crate::domain::todo::Todo;

#[derive(Deserialize, Debug)]
pub struct ListTodosResponse {
    pub todos: Vec<Todo>,
}

pub async fn list_todos() -> Result<ListTodosResponse> {
    reqwest::get("http://localhost:18080/todos")
        .await
        .unwrap()
        .json::<ListTodosResponse>()
        .await
}

#[derive(Deserialize, Debug)]
pub struct GetTodoResponse {
    pub todo: Todo,
}

pub async fn get_todo(id: String) -> Result<GetTodoResponse> {
    reqwest::get(format!("http://localhost:18080/todos/{}", id))
        .await
        .unwrap()
        .json::<GetTodoResponse>()
        .await
}
