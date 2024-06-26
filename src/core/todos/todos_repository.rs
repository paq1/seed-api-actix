use async_trait::async_trait;

#[async_trait]
pub trait TodosRepository<ENTITY, ID> {
    async fn fetchOne(&self, id: ID) -> Result<Option<ENTITY>, String>;
    async fn insert(&self, entity: ENTITY) -> Result<ID, String>;

}