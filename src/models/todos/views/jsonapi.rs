use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Many<T>
where
    T: Serialize + Clone
{
    #[schema(example = "[]")]
    pub items: Vec<T>,
}
