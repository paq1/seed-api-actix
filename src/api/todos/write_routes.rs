use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use futures::lock::Mutex;

use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::JwtTokenService;
use crate::api::todos::services::TodosServiceImpl;
use crate::api::todos::todo_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::core::todos::services::TodosService;
use crate::models::shared::errors::StandardHttpError;
use crate::models::todos::commands::{CreateTodoCommand, UpdateTodoCommand};
use crate::models::todos::views::Todo;

#[utoipa::path(
    request_body = CreateTodoCommand,
    responses(
    (status = 201, description = "fait ca", body = Todo),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/todos")]
pub async fn insert_one(
    req: HttpRequest,
    body: web::Json<CreateTodoCommand>,
    jwt_token_service: web::Data<JwtTokenService>,
    todos_service: web::Data<Arc<Mutex<TodosServiceImpl<TodosMongoRepository, TodosEventMongoRepository>>>>,
    http_error: web::Data<StandardHttpError>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()) {
        Ok(ctx) => {
            let command = body.into_inner();
            let lock = todos_service.lock().await;

            let result_insert = lock.create_todo(command, ctx).await;

            match result_insert {
                Ok(res) => HttpResponse::Created().json(Todo { name: res }),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

#[utoipa::path(
    request_body = UpdateTodoCommand,
    responses(
    (status = 200, description = "fait ca", body = Todo),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/todos/commands/update/{id}")]
pub async fn update_one(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateTodoCommand>,
    jwt_token_service: web::Data<JwtTokenService>,
    todos_service: web::Data<Arc<Mutex<TodosServiceImpl<TodosMongoRepository, TodosEventMongoRepository>>>>,
    http_error: web::Data<StandardHttpError>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()) {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = body.into_inner();
            let lock = todos_service.lock().await;

            let result_insert = lock.update_todo(command, id, ctx).await;

            match result_insert {
                Ok(res) => HttpResponse::Ok().json(Todo { name: res }),
                Err(_) => HttpResponse::InternalServerError().json(http_error.internal_server_error.clone())
            }
        }
        Err(_err) => HttpResponse::Unauthorized().json(http_error.unauthorized.clone())
    }
}

