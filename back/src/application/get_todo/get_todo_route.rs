use actix_web::{Responder, web};
use crate::AppState;

pub async fn get_todo(id: web::Path<String>, data: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let use_case = &data.get_todo_use_case;
    let todo = use_case.act(&id);
    Ok(web::Json(todo))
}
