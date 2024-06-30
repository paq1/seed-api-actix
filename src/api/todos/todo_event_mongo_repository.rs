use async_trait::async_trait;

use crate::api::shared::daos::dbos::EventDBO;
use crate::api::todos::todo_dbo::TodoDboEvent;
use crate::api::todos::todos_mongo_dao::TodosEventMongoDAO;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::EntityEvent;
use crate::core::todos::data::TodoEvents;
use crate::core::todos::todos_repository::{TodosEventRepositoryReadOnly, TodosEventRepositoryWriteOnly};

pub struct TodosEventMongoRepository {
    pub dao: TodosEventMongoDAO,
}

#[async_trait]
impl TodosEventRepositoryReadOnly for TodosEventMongoRepository {
    async fn fetch_one(&self, event_id: String) -> Result<Option<EntityEvent<TodoEvents, String>>, String> {
        self.dao.fetch_one(event_id).await.map(|maybevent| {
            maybevent.map(|event_dbo| {
                event_dbo.into()
            })
        })
    }
}

impl CanGetId<String> for EventDBO<TodoDboEvent, String> {
    fn id(&self) -> &String {
        &self.event_id
    }
}

#[async_trait]
impl TodosEventRepositoryWriteOnly for TodosEventMongoRepository {
    async fn insert_one(&self, todo: EntityEvent<TodoEvents, String>) -> Result<String, String> {
        let dao: EventDBO<TodoDboEvent, String> = todo.into();

        let dao_sanitize_version: EventDBO<TodoDboEvent, String> = EventDBO {
            version: Some(0),
            ..dao.clone()
        };

        self.dao.insert(dao_sanitize_version).await
    }
}