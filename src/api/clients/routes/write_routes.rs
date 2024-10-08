use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, post, put, Responder, web};
use uuid::Uuid;
use crate::api::shared::helpers::http_response::{CanToHttpResponse, HttpKindResponse};
use crate::api::shared::mappers::event_api_view::from_entity_event_to_view;
use crate::api::shared::token::authenticated::authenticated;
use crate::api::shared::token::services::jwt_hmac::JwtHMACTokenService;
use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::event_sourcing::engine::Engine;
use crate::models::clients::commands::{ClientsCommands, CreateClientCommand, DisableClientCommand, UpdateClientCommand};
use crate::models::clients::views::ClientViewEvent;
use crate::models::shared::errors::StandardHttpError;

#[utoipa::path(
    request_body = CreateClientCommand,
    responses(
    (status = 201, description = "fait ca", body = ClientView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[post("/clients/commands/create")]
pub async fn insert_one_client(
    req: HttpRequest,
    body: web::Json<CreateClientCommand>,
    jwt_token_service: web::Data<JwtHMACTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let command = ClientsCommands::Create(body.into_inner());

            let entity_id = Uuid::new_v4().to_string();

            let event = engine
                .compute(command, entity_id, "create-client".to_string(), &ctx).await;

            event.map(|(event, _)| {
                from_entity_event_to_view::<ClientEvents, ClientViewEvent>(
                    event,
                    "clients".to_string(),
                    "org:example:insurance:client".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Created)
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

#[utoipa::path(
    request_body = UpdateClientCommand,
    responses(
    (status = 200, description = "fait ca", body = ClientView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/clients/{entity_id}/commands/update")]
pub async fn update_one_client(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<UpdateClientCommand>,
    jwt_token_service: web::Data<JwtHMACTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = ClientsCommands::Update(body.into_inner());

            let event = engine
                .compute(command, id, "update-client".to_string(), &ctx).await;

            event.map(|(event, _)| {
                from_entity_event_to_view::<ClientEvents, ClientViewEvent>(
                    event,
                    "clients".to_string(),
                    "org:example:insurance:client".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Ok)
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

#[utoipa::path(
    request_body = DisableClientCommand,
    responses(
    (status = 200, description = "fait ca", body = ClientView),
    ),
    security(
    ("bearer_auth" = [])
    )
)]
#[put("/clients/{entity_id}/commands/disable")]
pub async fn disable_one_client(
    path: web::Path<String>,
    req: HttpRequest,
    body: web::Json<DisableClientCommand>,
    jwt_token_service: web::Data<JwtHMACTokenService>,
    http_error: web::Data<StandardHttpError>,
    engine: web::Data<Arc<Engine<ClientStates, ClientsCommands, ClientEvents>>>,
) -> impl Responder {
    match authenticated(&req, jwt_token_service.get_ref()).await {
        Ok(ctx) => {
            let id = path.into_inner();
            let command = ClientsCommands::Disable(body.into_inner());

            let event = engine
                .compute(command, id, "disable-client".to_string(), &ctx).await;


            event.map(|(event, _)| {
                from_entity_event_to_view::<ClientEvents, ClientViewEvent>(
                    event,
                    "clients".to_string(),
                    "org:example:insurance:client".to_string(),
                    &ctx,
                )
            })
                .to_http_response_with_error_mapping(HttpKindResponse::Ok)
        }
        Err(_err) => HttpResponse::Unauthorized().json(&http_error.unauthorized)
    }
}

