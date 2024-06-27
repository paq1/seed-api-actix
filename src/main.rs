mod core;
mod api;
mod models;

use std::sync::{Arc, Mutex};
use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use crate::api::todos::todos_mongo_repository::TodosMongoRepository;

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TokenClaims {
    id: i32
}


#[derive(Serialize, Deserialize, Clone, ToSchema)]
struct Todo {
    #[schema(example = "tache du jour")]
    name: String,
}


#[derive(Serialize, Deserialize, Clone, ToSchema)]
struct Many<T>
    where
        T: Serialize + Clone
{
    #[schema(example = "xxx")]
    items: Vec<T>,
}


#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Many<Todo>)
    )
)]
#[get("/todos")]
async fn fetch_many() -> impl Responder {
    HttpResponse::Ok().json(Many::<Todo> { items: vec![] })
}

#[utoipa::path(
    responses(
        (status = 200, description = "fait ca", body = Todo)
    )
)]
#[get("/todos/{id}")]
async fn fetch_one(path: web::Path<String>, service_test: web::Data<Arc<Mutex<TodosMongoRepository>>>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(Todo { name: id })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let repo: Arc<Mutex<TodosMongoRepository>> = Arc::new(Mutex::new(TodosMongoRepository::new().await));

    #[derive(utoipa::OpenApi)]
    #[openapi(
        paths(
            fetch_many,
            fetch_one
        ),
        components(
            schemas(
                TokenClaims,
                Many<Todo>,
                Todo
            )
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build()
                )
            )
        }
    }

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials();

        App::new()
            .app_data(
                web::Data::new(repo.clone())
            )
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .service(fetch_one)
            .service(fetch_many)
    })
        .workers(1)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}