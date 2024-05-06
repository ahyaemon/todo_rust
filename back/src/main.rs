use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header};
use dotenvy_macro::dotenv;

mod domain;
mod application;
mod adapter;

use crate::application::get_todo::get_todo_use_case::GetTodoUseCase;
use crate::application::add_todo::add_todo_route::create_todo;
use crate::application::add_todo::add_todo_use_case::AddTodoUseCase;
use crate::application::delete_todo::delete_todo_route::delete_todo;
use crate::application::delete_todo::delete_todo_use_case::DeleteTodoUseCase;
use crate::application::get_todo::get_todo_route::get_todo;
use crate::application::list_todo::list_todos_use_case::ListTodosUseCase;
use crate::application::list_todo::list_todo_route::list_todos;
use crate::adapter::todo_repository_impl::TodoRepositoryImpl;

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
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .service(health_check)
            .service(
                web::scope("/todos")
                    .route("/{id}", web::get().to(get_todo))
                    .route("", web::get().to(list_todos))
                    .route("/{id}", web::delete().to(delete_todo))
                    .route("", web::put().to(create_todo))
            )
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
