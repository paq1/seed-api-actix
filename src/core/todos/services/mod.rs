use async_trait::async_trait;
use crate::core::shared::context::Context;
use crate::models::todos::commands::CreateTodo;

#[async_trait]
pub trait TodosService {
    async fn create_todo(&self, command: CreateTodo, ctx: Context) -> Result<String, String>;
}