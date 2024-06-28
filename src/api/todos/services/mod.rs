use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

use crate::core::shared::data::Entity;
use crate::core::todos::data::{Todo, TodoStates};
use crate::core::todos::services::TodosService;
use crate::core::todos::todos_repository::TodosRepositoryWriteOnly;
use crate::models::todos::commands::CreateTodo;

pub struct TodosServiceImpl<STORE>
where
    STORE: TodosRepositoryWriteOnly,
{
    pub store: Arc<Mutex<STORE>>,
}

#[async_trait]
impl<STORE> TodosService for TodosServiceImpl<STORE>
where
    STORE: TodosRepositoryWriteOnly + Send,
{
    async fn create_todo(&self, command: CreateTodo) -> Result<String, String> {
        let entity: Entity<TodoStates, String> = Entity {
            entity_id: "xxx".to_string(),
            data: TodoStates::Todo(Todo { name: command.name }),
        };

        Arc::clone(&self.store)
            .lock().await
            .insert_one(entity).await
    }
}