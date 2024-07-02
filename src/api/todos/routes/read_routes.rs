use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::Query;
use futures::lock::Mutex;

use crate::api::shared::query::pagination::HttpPaginationQuery;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::shared::repositories::query::{PaginationDef, Query as QueryDef};
use crate::core::shared::repositories::{ReadOnlyEntityRepo, ReadRepoWithPagination};
use crate::models::shared::errors::StandardHttpError;
use crate::models::shared::jsonapi::Many;

impl From<Query<HttpPaginationQuery>> for QueryDef {
    fn from(value: Query<HttpPaginationQuery>) -> Self {
        let size = value.size.unwrap_or(10);
        let number = value.number.unwrap_or(1);

        Self {
            pagination: PaginationDef {
                page_number: number,
                page_size: size
            }
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many < Todo >)
    ),
    params(
        HttpPaginationQuery,
    )
)]
#[get("/todos")]
pub async fn fetch_many(
    store: web::Data<Arc<Mutex<TodosMongoRepository>>>,
    http_error: web::Data<StandardHttpError>,
    query: Query<HttpPaginationQuery>,
) -> impl Responder {

    println!("query {query:?}");
    let store_lock = store.lock().await;
    match store_lock.fetch_many(query.into()).await {
        Ok(items) => HttpResponse::Ok().json(Many::new(items)),
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
