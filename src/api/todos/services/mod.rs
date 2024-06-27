use std::sync::{Arc};
use futures::lock::Mutex;
use async_trait::async_trait;
use crate::api::shared::repository::dbos::Entity;
use crate::api::todos::todo_dbo::TodoDbo;
use crate::core::shared::repositories::repository::WriteOnlyRepository;
use crate::core::todos::services::TodosService;
use crate::models::todos::commands::CreateTodo;

pub struct TodosServiceImpl<STORE>
where
    STORE: WriteOnlyRepository<Entity<TodoDbo, String>, String>,
{
    pub store: Arc<Mutex<STORE>>,
}

#[async_trait]
impl<STORE> TodosService for TodosServiceImpl<STORE>
where
    STORE: WriteOnlyRepository<Entity<TodoDbo, String>, String> + Send,
{
    async fn create_todo(&self, command: CreateTodo) -> Result<String, String> {
        let entity: Entity<TodoDbo, String> = Entity {
            id_mongo: None,
            version: 0,
            entity_id: "xxx".to_string(),
            data: TodoDbo { name: command.name },
        };

        Arc::clone(&self.store)
            .lock().await
            .insert(entity).await
    }
}