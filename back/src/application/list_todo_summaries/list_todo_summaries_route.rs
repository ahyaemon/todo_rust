use actix_web::{Responder, web, Result};
use serde::Serialize;
use crate::application::list_todo_summaries::list_todo_summaries_use_case::TodoSummary;
use crate::AppState;

#[derive(Serialize)]
struct ListTodoSummariesResponse {
    todo_summaries: Vec<TodoSummary>
}

pub async fn list_todo_summaries(data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.list_todo_summaries_use_case;
    let todo_summaries = use_case.act();
    let response = ListTodoSummariesResponse { todo_summaries };
    Ok(web::Json(response))
}
