use crate::domain::todo::Todo;
use crate::AppState;
use actix_web::{web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct GetTodoResponse {
    todo: Todo,
}

pub async fn get_todo(id: web::Path<String>, data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.get_todo_use_case;
    let todo = use_case.act(&id);
    let response = GetTodoResponse { todo };
    Ok(web::Json(response))
}
