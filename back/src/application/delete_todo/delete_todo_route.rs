use crate::AppState;
use actix_web::{web, Responder};

pub async fn delete_todo(
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let use_case = &data.delete_todo_use_case;
    use_case.act(&id);
    Ok("OK")
}
