use actix_web::{Responder, web, Result};
use serde::Serialize;
use crate::AppState;
use crate::domain::todo::Todo;

#[derive(Serialize)]
struct GetTodoResponse {
    todo: Todo
}

pub async fn get_todo(id: web::Path<String>, data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.get_todo_use_case;
    let todo = use_case.act(&id);
    let response = GetTodoResponse { todo };
    Ok(web::Json(response))
}
