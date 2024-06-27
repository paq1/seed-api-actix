use std::sync::{Arc, Mutex};
use actix_web::{get, HttpResponse, Responder, web};
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::models::todos::views::jsonapi::Many;
use crate::models::todos::views::Todo;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many<Todo>)
    )
)]
#[get("/todos")]
pub async fn fetch_many() -> impl Responder {
    HttpResponse::Ok().json(Many::<Todo> { items: vec![] })
}

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Todo)
    )
)]
#[get("/todos/{id}")]
pub async fn fetch_one(path: web::Path<String>, service_test: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(Todo { name: id })
}
