use async_trait::async_trait;
use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait ReadOnlyDAO<DBO, ID> {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<DBO>>;
    async fn fetch_all(&self) -> ResultErr<Vec<DBO>>;
}

#[async_trait]
pub trait WriteOnlyDAO<DBO, ID> {
    async fn insert(&self, entity: DBO) -> ResultErr<ID>;
    async fn update(&self, id: ID, entity: DBO) -> ResultErr<ID>;
    async fn delete(&self, id: ID) -> ResultErr<ID>;
}

