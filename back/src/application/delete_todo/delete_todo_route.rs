use actix_web::{Responder, web};
use crate::AppState;

pub async fn delete_todo(
    id: web::Path<String>,
    data: web::Data<AppState>
) -> actix_web::Result<impl Responder> {
    let use_case = &data.delete_todo_use_case;
    use_case.act(&id);
    Ok("OK")
}
