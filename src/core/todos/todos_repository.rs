use async_trait::async_trait;

use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::todos::data::{TodoEvents, TodoStates};

#[async_trait]
pub trait TodosRepositoryReadOnly {
    async fn fetch_one(&self, id: String) -> Result<Option<Entity<TodoStates, String>>, String>;
    async fn fetch_all(&self) -> Result<Vec<Entity<TodoStates, String>>, String>;
}

#[async_trait]
pub trait TodosRepositoryWriteOnly {
    async fn insert_one(&self, todo: Entity<TodoStates, String>) -> Result<String, String>;
}

#[async_trait]
pub trait TodosEventRepositoryReadOnly {
    async fn fetch_one(&self, event_id: String) -> Result<Option<EntityEvent<TodoEvents, String>>, String>;
}

#[async_trait]
pub trait TodosEventRepositoryWriteOnly {
    async fn insert_one(&self, todo: EntityEvent<TodoEvents, String>) -> Result<String, String>;
}