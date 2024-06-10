use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    name: String,
}


#[derive(Serialize, Deserialize, Clone)]
struct Many<T>
    where
        T: Serialize + Clone
{
    items: Vec<T>,
}


#[get("/todos")]
async fn fetch_many() -> impl Responder {
    HttpResponse::Ok().json(Many::<Todo> { items: vec![] })
}

#[get("/todos/{id}")]
async fn fetch_one(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(Todo { name: id })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials();
        App::new()
            .wrap(cors)
            .service(fetch_one)
            .service(fetch_many)
    })
        .workers(1)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}