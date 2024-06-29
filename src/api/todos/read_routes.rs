use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use futures::lock::Mutex;

use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::todos::todos_repository::TodosRepositoryReadOnly;
use crate::models::todos::views::errors::Error;
use crate::models::todos::views::jsonapi::Many;
use crate::models::todos::views::Todo;

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < Todo >)
    )
)]
#[get("/todos")]
pub async fn fetch_many() -> impl Responder {
    HttpResponse::Ok().json(Many::<Todo> { items: vec![] })
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
