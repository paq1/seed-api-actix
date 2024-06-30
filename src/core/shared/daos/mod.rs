use async_trait::async_trait;

#[async_trait]
pub trait ReadOnlyDAO<DBO, ID> {
    async fn fetch_one(&self, id: ID) -> Result<Option<DBO>, String>;
    async fn fetch_all(&self) -> Result<Vec<DBO>, String>;
}

#[async_trait]
pub trait WriteOnlyDAO<DBO, ID> {
    async fn insert(&self, entity: DBO) -> Result<ID, String>;
    async fn update(&self, id: ID, entity: DBO) -> Result<ID, String>;
    async fn delete(&self, id: ID) -> Result<ID, String>;
}
