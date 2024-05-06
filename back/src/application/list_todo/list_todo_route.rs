use actix_web::{Responder, web, Result};
use serde::Serialize;
use crate::AppState;
use crate::domain::todo::Todo;

#[derive(Serialize)]
struct ListTodosResponse {
    todos: Vec<Todo>
}

pub async fn list_todos(data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.list_todos_use_case;
    let todos = use_case.act();
    let response = ListTodosResponse { todos };
    Ok(web::Json(response))
}
