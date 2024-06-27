use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct CreateTodo {
    #[schema(example = "input")]
    pub name: String,
}