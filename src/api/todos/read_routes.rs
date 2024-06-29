use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use futures::lock::Mutex;

use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::todos::todos_repository::TodosRepositoryReadOnly;
use crate::models::todos::views::errors::Error;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < Todo >)
    )
)]
#[get("/todos")]
pub async fn fetch_many(store: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let store_lock = store.lock().await;
    match store_lock.fetch_all().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().json(Error {title: err})
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
pub async fn fetch_one(path: web::Path<String>, repo: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let id = path.into_inner();

    let repo_lock = repo.lock().await;

    match repo_lock.fetch_one(id).await {
        Ok(Some(res)) => HttpResponse::Ok().json(res.clone()), // fixme mettre une vue ici
        Ok(_) => HttpResponse::NotFound().json(Error {title: "pas de data".to_string()}),
        Err(err) => HttpResponse::InternalServerError().json(Error {title: err})
    }
}
