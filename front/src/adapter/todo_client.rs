use serde::Deserialize;
use reqwest::Result;
use reqwest::Client;
use crate::domain::todo::Todo;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TodoSummary {
    pub id: String,
    pub title: String,
    pub description_summary: String,
}

#[derive(Deserialize, Debug)]
pub struct ListTodosResponse {
    pub todo_summaries: Vec<TodoSummary>,
}

pub async fn list_todo_summaries() -> Result<ListTodosResponse> {
    reqwest::get("http://localhost:18080/todos/summaries")
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

#[derive(Deserialize, Debug)]
pub struct AddTodoResponse {
    pub todo: Todo,
}

pub async fn add_todo(title: &str) -> Result<AddTodoResponse> {
    let client = Client::new();
    let json = &serde_json::json!({
        "title": title,
    });
    client
        .put("http://localhost:18080/todos")
        .json(json)
        .send()
        .await
        .unwrap()
        .json::<AddTodoResponse>()
        .await
}
