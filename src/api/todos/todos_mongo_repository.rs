use async_trait::async_trait;

use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::todos::todo_dbo::TodoDboState;
use crate::api::todos::todos_mongo_dao::TodosMongoDAO;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::Entity;
use crate::core::todos::data::TodoStates;
use crate::core::todos::todos_repository::{TodosRepositoryReadOnly, TodosRepositoryWriteOnly};

pub struct TodosMongoRepository {
    pub dao: TodosMongoDAO,
}

#[async_trait]
impl TodosRepositoryReadOnly for TodosMongoRepository {
    async fn fetch_one(&self, id: String) -> Result<Option<Entity<TodoStates, String>>, String> {
        self.dao
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|x| Entity { entity_id: x.entity_id, data: x.data.into() }))
    }

    async fn fetch_all(&self) -> Result<Vec<Entity<TodoStates, String>>, String> {
        self.dao
            .fetch_all()
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| Entity { entity_id: dbo.entity_id, data: dbo.data.into() })
                    .collect()
            })
    }
}

impl CanGetId<String> for EntityDBO<TodoDboState, String> {
    fn id(&self) -> String {
        self.entity_id.clone()
    }
}

#[async_trait]
impl TodosRepositoryWriteOnly for TodosMongoRepository {
    async fn insert_one(&self, todo: Entity<TodoStates, String>) -> Result<String, String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = todo.into();

        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo.clone()
        };

        self.dao.insert(sanitize_version).await
    }
}
