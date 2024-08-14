use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use crate::api::clients::routes::read_routes::__path_fetch_events_client;
use crate::api::clients::routes::read_routes::__path_fetch_many_client;
use crate::api::clients::routes::read_routes::__path_fetch_one_client;
use crate::api::clients::routes::write_routes::__path_disable_one_client;
use crate::api::clients::routes::write_routes::__path_insert_one_client;
use crate::api::clients::routes::write_routes::__path_update_one_client;
use crate::core::shared::repositories::query::{InfoPaged, Page, Paged};
use crate::models::clients::commands::*;
use crate::models::clients::shared::{ClientData, DisableReason};
use crate::models::clients::views::ClientView;
use crate::models::clients::views::ClientViewEvent;
use crate::models::clients::views::*;
use crate::models::shared::jsonapi::ManyView;
use crate::models::shared::views::command_handler_view::ApiView;
use crate::models::shared::views::entities::EntityView;
use crate::models::shared::views::DataWrapperView;
use crate::models::shared_business::Adresse;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        fetch_many_client,
        fetch_one_client,
        insert_one_client,
        update_one_client,
        disable_one_client,
        fetch_events_client,
    ),
    components(
        schemas(
            ClientView,
            ManyView < ClientViewState >,
            CreateClientCommand,
            UpdateClientCommand,
            DisableClientCommand,
            ClientData,
            Adresse,
            DisableReason,
            DataWrapperView < ApiView < ClientViewEvent > >,
            EntityView<ClientViewState>,
            Paged<EntityView<ClientViewState>>,
            InfoPaged,
            Page,
        )
    ),
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
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
            ),
        )
    }
}