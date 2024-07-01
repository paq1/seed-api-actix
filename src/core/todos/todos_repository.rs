use async_trait::async_trait;

use crate::core::shared::data::EntityEvent;
use crate::core::todos::data::TodoEvents;

#[async_trait]
pub trait TodosEventRepositoryReadOnly {
    async fn fetch_one(&self, event_id: String) -> Result<Option<EntityEvent<TodoEvents, String>>, String>;
}

#[async_trait]
pub trait TodosEventRepositoryWriteOnly {
    async fn insert_one(&self, todo: EntityEvent<TodoEvents, String>) -> Result<String, String>;
}