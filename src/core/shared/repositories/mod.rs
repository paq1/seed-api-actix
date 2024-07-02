use async_trait::async_trait;

use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::repositories::query::{Paged, Query};
use crate::models::shared::errors::ResultErr;

pub mod query;

#[async_trait]
pub trait ReadOnlyEntityRepo<DATA: Clone, ID: Clone> {
    async fn fetch_one(&self, id: ID) -> ResultErr<Option<Entity<DATA, ID>>>;
    async fn fetch_all(&self) -> ResultErr<Vec<Entity<DATA, ID>>>;

    async fn fetch_many(&self, query: Query) -> ResultErr<Paged<Entity<DATA, ID>>> {
        let entities = self.fetch_all().await?;
        let start = (query.pagination.page_number - 1) * query.pagination.page_size;
        let end = start.clone() + query.pagination.page_size;

        let paged_entities = if entities.is_empty() {
            vec![]
        } else {
            if start > entities.len() {
                vec![]
            } else {
                let sanitize_end = if end > entities.len() {
                    entities.len()
                } else {
                    end
                };
                entities.clone().iter().as_slice()[start..sanitize_end].to_vec()
            }
        };

        Ok(
            Paged {
                data: paged_entities,
                meta: query.pagination
            }
        )

    }
}

#[async_trait]
pub trait WriteOnlyEntityRepo<DATA: Clone, ID: Clone> {
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
