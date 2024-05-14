use crate::domain::todo::Todo;
use crate::AppState;
use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddTodoRequest {
    title: String,
    description: String,
}

#[derive(Serialize)]
pub struct AddTodoResponse {
    todo: Todo,
}

pub async fn create_todo(
    request: web::Json<AddTodoRequest>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let use_case = &data.add_todo_use_case;
    let todo = use_case.act(request.title.clone(), request.description.clone());
    let response = AddTodoResponse { todo };
    Ok(web::Json(response))
}
