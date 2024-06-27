use std::sync::{Arc, Mutex};

use actix_web::{get, HttpResponse, post, Responder, web};
use crate::api::shared::repository::dbos::Entity;
use crate::api::todos::todo_dbo::TodoDbo;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::shared::repositories::repository::WriteOnlyRepository;
use crate::models::todos::commands::CreateTodo;
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
        (status = 200, description = "fait ca", body = Todo)
    )
)]
#[get("/todos/{id}")]
pub async fn fetch_one(path: web::Path<String>, service_test: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(Todo { name: id })
}

#[utoipa::path(
    request_body = CreateTodo,
    responses(
    (status = 201, description = "fait ca", body = Todo),
    )
)]
#[post("/todos")]
pub async fn insert_one(body: web::Json<CreateTodo>, service_test: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let command = body.into_inner();
    let lock = service_test.lock();
    let entity: Entity<TodoDbo, String> = Entity {
        id_mongo: None,
        version: 0,
        entity_id: "xxx".to_string(),
        data: TodoDbo { name: command.name },
    };

    let result_insert = lock.unwrap().insert(entity).await;

    match result_insert {
        Ok(res) => HttpResponse::Created().json(Todo { name: res }),
        Err(err) => HttpResponse::InternalServerError().json(Error { title: err })
    }
}
