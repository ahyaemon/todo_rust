use crate::domain::todo::Todo;
use crate::AppState;
use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ListTodosResponse {
    todos: Vec<Todo>,
}

pub async fn list_todos(data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.list_todos_use_case;
    let todos = use_case.act();
    let response = ListTodosResponse { todos };
    Ok(web::Json(response))
}
