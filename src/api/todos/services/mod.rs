use std::sync::Arc;
use std::time::Instant;
use async_trait::async_trait;
use futures::lock::Mutex;

use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::todos::data::{TodoEvents, TodoStates};
use crate::core::todos::services::TodosService;
use crate::core::todos::todos_repository::{TodosEventRepositoryWriteOnly, TodosRepositoryWriteOnly};
use crate::models::todos::commands::CreateTodo;

pub struct TodosServiceImpl<STORE, JOURNAL>
where
    STORE: TodosRepositoryWriteOnly,
    JOURNAL: TodosEventRepositoryWriteOnly,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<STORE, JOURNAL> TodosService for TodosServiceImpl<STORE, JOURNAL>
where
    STORE: TodosRepositoryWriteOnly + Send,
    JOURNAL: TodosEventRepositoryWriteOnly + Send,
{
    async fn create_todo(&self, command: CreateTodo) -> Result<String, String> {

        // fixme mettre des erreurs standard: String -> CustomError / Failure
        let entity: Entity<TodoStates, String> = Entity {
            entity_id: "xxx".to_string(), // fixme genere
            data: TodoStates::Todo { name: command.name },
        };

        let event: EntityEvent<TodoEvents, String> = EntityEvent {
            entity_id: "xxx".to_string(), // fixme genere
            event_id: "www".to_string(),
            data: TodoEvents::Created { by: "mkd".to_string(), at: "xxx".to_string() },
        };


        let insert_journal = Arc::clone(&self.journal)
            .lock().await
            .insert_one(event).await;

        let store = Arc::clone(&self.store)
            .lock().await
            .insert_one(entity).await;

        insert_journal.and_then(|_| store)
    }
}