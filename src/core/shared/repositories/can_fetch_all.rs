use async_trait::async_trait;

use crate::models::shared::errors::ResultErr;

#[async_trait]
pub trait CanFetchAll<DATA: Clone> {
    async fn fetch_all(&self) -> ResultErr<Vec<DATA>>;
}