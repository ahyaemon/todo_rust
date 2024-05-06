use actix_web::{Responder, web};
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    title: String,
}

pub async fn create_todo(
    request: web::Json<CreateTodoRequest>,
    data: web::Data<AppState>
) -> actix_web::Result<impl Responder> {
    let use_case = &data.add_todo_use_case;
    let todo = use_case.act(&request.title);
    Ok(web::Json(todo))
}
