use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod jsonapi;
pub mod errors;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Todo {
    #[schema(example = "tache du jour")]
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TokenClaims {
    pub id: i32
}
