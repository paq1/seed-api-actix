use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::core::shared::context::Context;
use crate::core::shared::daos::{ReadOnlyEntityRepo, WriteOnlyEntityRepo};
use crate::core::shared::data::{Entity, EntityEvent};
use crate::core::shared::id_generator::IdGenerator;
use crate::core::todos::data::{TodoEvents, TodoStates, UpdatedEvent};
use crate::core::todos::data::TodoStates::Todo;
use crate::core::todos::services::TodosService;
use crate::core::todos::todos_repository::TodosEventRepositoryWriteOnly;
use crate::models::todos::commands::*;

pub struct TodosServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<TodoStates, String> + ReadOnlyEntityRepo<TodoStates, String>,
    JOURNAL: TodosEventRepositoryWriteOnly,
{
    pub store: Arc<Mutex<STORE>>,
    pub journal: Arc<Mutex<JOURNAL>>,
}

#[async_trait]
impl<STORE, JOURNAL> TodosService for TodosServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<TodoStates, String> + ReadOnlyEntityRepo<TodoStates, String> + Send,
    JOURNAL: TodosEventRepositoryWriteOnly + Send,
{
    async fn create_todo(&self, command: CreateTodoCommand, context: Context) -> Result<String, String> {

        // fixme mettre des erreurs standard: String -> CustomError / Failure
        let entity_id = Self::generate_id();
        let event_id = Self::generate_id();

        let entity: Entity<TodoStates, String> = Entity {
            entity_id: entity_id.clone(),
            data: TodoStates::Todo { name: command.name.clone() },
            version: None,
        };

        let event: EntityEvent<TodoEvents, String> = EntityEvent {
            entity_id: entity_id.clone(),
            event_id: event_id.clone(),
            data: TodoEvents::Created { by: context.subject, at: context.now, name: command.name.clone() },
        };


        let insert_journal = Arc::clone(&self.journal)
            .lock().await
            .insert_one(event).await;

        let store = Arc::clone(&self.store)
            .lock().await
            .insert(entity).await;

        insert_journal.and_then(|_| store)
    }

    async fn update_todo(&self, command: UpdateTodoCommand, id: String, ctx: Context) -> Result<String, String> {
        let current = self.store.lock().await.fetch_one(id.clone()).await?;

        match current {
            Some(entity) => {
                let event_id = Self::generate_id();

                let event: EntityEvent<TodoEvents, String> = EntityEvent {
                    entity_id: id.clone(),
                    event_id: event_id.clone(),
                    data: TodoEvents::Updated(UpdatedEvent { by: ctx.subject, at: ctx.now, name: command.name.clone() }),
                };

                let update_state = self.store.lock().await
                    .update(
                        id.clone(),
                        Entity {
                            data:
                            match entity.data.clone() {
                                _ => {
                                    Todo {
                                        name: command.name
                                    }
                                }
                            },
                            ..entity.clone()
                        }
                    ).await;

                self.journal.lock().await.insert_one(event).await.and_then(|_| update_state)
            },
            None => Err("not found".to_string())
        }
    }

    async fn delete_todo(&self, _command: DeleteTodoCommand, _id: String, _ctx: Context) -> Result<String, String> {
        todo!()
    }
}

impl<STORE, JOURNAL> IdGenerator for TodosServiceImpl<STORE, JOURNAL>
where
    STORE: WriteOnlyEntityRepo<TodoStates, String> + ReadOnlyEntityRepo<TodoStates, String>,
    JOURNAL: TodosEventRepositoryWriteOnly
{
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}
