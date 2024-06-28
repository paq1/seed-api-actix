use async_trait::async_trait;

use crate::models::todos::commands::CreateTodo;

#[async_trait]
pub trait TodosService {
    async fn create_todo(&self, command: CreateTodo) -> Result<String, String>;
}