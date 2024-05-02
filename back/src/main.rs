use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, put, delete};
use actix_web::{Result};
use dotenvy_macro::dotenv;
use serde::Deserialize;

mod domain;
mod application;
mod adapter;

use crate:: {
    application::{
        get_todo_use_case::GetTodoUseCase,
    },
    adapter::{
        todo_repository_impl::TodoRepositoryImpl,
    },
};
use crate::application::add_todo_use_case::AddTodoUseCase;
use crate::application::delete_todo_use_case::DeleteTodoUseCase;
use crate::application::list_todos_use_case::ListTodosUseCase;

struct AppState {
    get_todo_use_case: GetTodoUseCase<TodoRepositoryImpl>,
    list_todos_use_case: ListTodosUseCase<TodoRepositoryImpl>,
    add_todo_use_case: AddTodoUseCase<TodoRepositoryImpl>,
    delete_todo_use_case: DeleteTodoUseCase<TodoRepositoryImpl>,
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[get("/todo/{id}")]
async fn get_todo(id: web::Path<String>, data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.get_todo_use_case;
    let todo = use_case.act(&id);
    Ok(web::Json(todo))
}

#[get("/todos")]
async fn list_todos(data: web::Data<AppState>) -> Result<impl Responder> {
    let use_case = &data.list_todos_use_case;
    let todos = use_case.act();
    Ok(web::Json(todos))
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
}

#[put("/todos")]
async fn create_todo(
    request: web::Json<CreateTodoRequest>,
    data: web::Data<AppState>
) -> Result<impl Responder> {
    let use_case = &data.add_todo_use_case;
    let todo = use_case.act(&request.title);
    Ok(web::Json(todo))
}

#[delete("/todos/{id}")]
async fn delete_todo(
    id: web::Path<String>,
    data: web::Data<AppState>
) -> Result<impl Responder> {
    let use_case = &data.delete_todo_use_case;
    use_case.act(&id);
    Ok("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hash_map = Mutex::new(HashMap::new());
    let repository = Arc::new(TodoRepositoryImpl::new(hash_map));
    let app_data = web::Data::new(AppState {
        get_todo_use_case: GetTodoUseCase::new(repository.clone()),
        list_todos_use_case: ListTodosUseCase::new(repository.clone()),
        add_todo_use_case: AddTodoUseCase::new(repository.clone()),
        delete_todo_use_case: DeleteTodoUseCase::new(repository.clone()),
    });
    let port: u16 = dotenv!("PORT").parse().expect("Env var PORT is not set.");
    println!("use port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(health_check)
            .service(get_todo)
            .service(list_todos)
            .service(create_todo)
            .service(delete_todo)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
