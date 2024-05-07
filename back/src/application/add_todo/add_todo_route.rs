use actix_web::{Responder, web};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::domain::todo::Todo;

#[derive(Deserialize)]
pub struct AddTodoRequest {
    title: String,
}

#[derive(Serialize)]
pub struct AddTodoResponse {
    todo: Todo,
}

pub async fn create_todo(
    request: web::Json<AddTodoRequest>,
    data: web::Data<AppState>
) -> actix_web::Result<impl Responder> {
    let use_case = &data.add_todo_use_case;
    let todo = use_case.act(&request.title);
    let response = AddTodoResponse{ todo };
    Ok(web::Json(response))
}
