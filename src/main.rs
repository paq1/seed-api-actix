use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use futures::lock::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::swagger::ApiDoc;
use crate::api::todos::read_routes::{fetch_many, fetch_one};
use crate::api::todos::services::TodosServiceImpl;
use crate::api::todos::todos_mongo_dao::TodosMongoDAO;
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;
use crate::api::todos::write_routes::insert_one;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo: Arc<Mutex<TodosMongoRepository>> = Arc::new(
        Mutex::new(
            TodosMongoRepository {
                dao: TodosMongoDAO::new("seedtodomongo".to_string(), "todos_store_actix".to_string()).await
            }
        )
    );

    let todos_service: Arc<Mutex<TodosServiceImpl<TodosMongoRepository>>> = Arc::new(
        Mutex::new(
            TodosServiceImpl {
                store: Arc::clone(&repo)
            }
        )
    );

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials();

        App::new()
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
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}