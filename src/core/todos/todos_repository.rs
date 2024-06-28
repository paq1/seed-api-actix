use async_trait::async_trait;

use crate::core::shared::data::Entity;
use crate::core::todos::data::{Todo, TodoStates};

#[async_trait]
pub trait TodosRepositoryReadOnly {
    async fn fetch_one(&self, id: String) -> Result<Option<Entity<TodoStates, String>>, String>;
}

#[async_trait]
pub trait TodosRepositoryWriteOnly {
    async fn insert_one(&self, todo: Entity<TodoStates, String>) -> Result<String, String>;
}