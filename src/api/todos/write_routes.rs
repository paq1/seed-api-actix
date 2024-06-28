use std::sync::Arc;

use actix_web::{HttpResponse, post, Responder, web};
use futures::lock::Mutex;

use crate::api::todos::services::TodosServiceImpl;
use crate::api::todos::todo_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::todos::services::TodosService;
use crate::models::todos::commands::CreateTodo;
use crate::models::todos::views::errors::Error;
use crate::models::todos::views::Todo;

#[utoipa::path(
    request_body = CreateTodo,
    responses(
    (status = 201, description = "fait ca", body = Todo),
    )
)]
#[post("/todos")]
pub async fn insert_one(
    body: web::Json<CreateTodo>,
    todos_service: web::Data<Arc<Mutex<TodosServiceImpl<TodosMongoRepository, TodosEventMongoRepository>>>>
) -> impl Responder {
    let command = body.into_inner();
    let lock = todos_service.lock().await;

    let result_insert = lock.create_todo(command).await;

    match result_insert {
        Ok(res) => HttpResponse::Created().json(Todo { name: res }),
        Err(err) => HttpResponse::InternalServerError().json(Error { title: err })
    }
}
