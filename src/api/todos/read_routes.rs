use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use futures::lock::Mutex;

use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::shared::daos::ReadOnlyEntityRepo;
use crate::models::shared::errors::StandardHttpError;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < Todo >)
    )
)]
#[get("/todos")]
pub async fn fetch_many(store: web::Data<Arc<Mutex<TodosMongoRepository>>>, http_error: web::Data<StandardHttpError>) -> impl Responder {
    let store_lock = store.lock().await;
    match store_lock.fetch_all().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}

#[utoipa::path(
    responses(
        (
          status = 200,
          description = "Get the current state.",
          body = Todo
        )
    )
)]
#[get("/todos/{id}")]
pub async fn fetch_one(path: web::Path<String>, repo: web::Data<Arc<Mutex<TodosMongoRepository>>>, http_error: web::Data<StandardHttpError>) -> impl Responder {
    let id = path.into_inner();

    let repo_lock = repo.lock().await;


    match repo_lock.fetch_one(id).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res.clone()),
        Ok(_) => HttpResponse::NotFound().json(http_error.not_found.clone()),
        Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
    }
}
