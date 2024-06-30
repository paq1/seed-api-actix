use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::shared::token::JwtTokenService;
use crate::api::swagger::ApiDoc;
use crate::api::todos::read_routes::{fetch_many, fetch_one};
use crate::api::todos::services::TodosServiceImpl;
use crate::api::todos::todo_event_mongo_repository::TodosEventMongoRepository;
use crate::api::todos::todos_mongo_dao::{TodosEventMongoDAO, TodosMongoDAO};
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::api::todos::write_routes::{insert_one, update_one};
use crate::models::shared::errors::StandardHttpError;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let repo: Arc<Mutex<TodosMongoRepository>> = Arc::new(
        Mutex::new(
            TodosMongoRepository {
                dao: TodosMongoDAO::new("seedtodomongo".to_string(), "todos_store_actix".to_string()).await
            }
        )
    );

    let journal: Arc<Mutex<TodosEventMongoRepository>> = Arc::new(
        Mutex::new(
            TodosEventMongoRepository {
                dao: TodosEventMongoDAO::new("seedtodomongo".to_string(), "todos_journal_actix".to_string()).await
            }
        )
    );

    let todos_service: Arc<Mutex<TodosServiceImpl<TodosMongoRepository, TodosEventMongoRepository>>> = Arc::new(
        Mutex::new(
            TodosServiceImpl {
                store: Arc::clone(&repo),
                journal: Arc::clone(&journal),
            }
        )
    );

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();
        let jwt_token_service = JwtTokenService::new("test".to_string());


        App::new()
            .app_data(web::Data::new(standard_http_error))
            .app_data(web::Data::new(jwt_token_service))
            .app_data(
                web::Data::new(Arc::clone(&repo))
            )
            .app_data(
                web::Data::new(Arc::clone(&todos_service))
            )
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .service(fetch_one)
            .service(fetch_many)
            .service(insert_one)
            .service(update_one)
    })
        .workers(2)
        .bind((api_address, api_port))?
        .run()
        .await
}