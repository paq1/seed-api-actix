use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Error {
    #[schema(example = "pouet")]
    pub title: String,
}