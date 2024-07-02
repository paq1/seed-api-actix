use async_trait::async_trait;
use crate::core::shared::data::{Entity, EntityEvent};
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait ReadOnlyEntityRepo<DATA, ID> {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<Entity<DATA, ID>>>;
    async fn fetch_all(&self) -> ResultErr<Vec<Entity<DATA, ID>>>;
}

#[async_trait]
pub trait WriteOnlyEntityRepo<DATA, ID> {
    async fn insert(&self, entity: Entity<DATA, ID>) -> ResultErr<ID>;
    async fn update(&self, id: ID, entity: Entity<DATA, ID>) -> ResultErr<ID>;
    async fn delete(&self, id: ID) -> ResultErr<ID>;
}

#[async_trait]
pub trait ReadOnlyEventRepo<DATA, ID> {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<EntityEvent<DATA, ID>>>;
    // async fn fetch_all(&self) -> Result<Vec<EntityEvent<DATA, ID>>, String>;
}

#[async_trait]
pub trait WriteOnlyEventRepo<DATA, ID> {
    async fn insert(&self, entity: EntityEvent<DATA, ID>) -> ResultErr<ID>;
    // async fn update(&self, id: ID, entity: Entity<DATA, ID>) -> Result<ID, String>;
    // async fn delete(&self, id: ID) -> Result<ID, String>;
}
