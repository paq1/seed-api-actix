use async_trait::async_trait;


#[async_trait]
pub trait ReadOnlyRepository<ENTITY, ID> {
    async fn fetch_one(&self, id: ID) -> Result<Option<ENTITY>, String>;
}

#[async_trait]
pub trait WriteOnlyRepository<ENTITY, ID> {
    async fn insert(&self, entity: ENTITY) -> Result<ID, String>;
}