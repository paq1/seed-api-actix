mod core;
mod api;
mod models;

use std::sync::{Arc, Mutex};
use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    name: String,
}


#[derive(Serialize, Deserialize, Clone)]
struct Many<T>
    where
        T: Serialize + Clone
{
    items: Vec<T>,
}


#[get("/todos")]
async fn fetch_many() -> impl Responder {
    HttpResponse::Ok().json(Many::<Todo> { items: vec![] })
}

struct ServiceImpl {
    x: i32
}

#[get("/todos/{id}")]
async fn fetch_one(path: web::Path<String>, service_test: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(Todo { name: id })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let repo: Arc<Mutex<TodosMongoRepository>> = Arc::new(Mutex::new(TodosMongoRepository::new().await));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials();

        App::new()
            .app_data(
                web::Data::new(repo.clone())
            )
            .wrap(cors)
            .service(fetch_one)
            .service(fetch_many)
    })
        .workers(1)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}