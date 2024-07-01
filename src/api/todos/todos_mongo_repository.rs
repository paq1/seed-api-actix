use async_trait::async_trait;

use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::todos::todo_dbo::TodoDboState;
use crate::api::todos::todos_mongo_dao::TodosMongoDAO;
use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, ReadOnlyEntityRepo, WriteOnlyDAO, WriteOnlyEntityRepo};
use crate::core::shared::data::Entity;
use crate::core::todos::data::TodoStates;

pub struct TodosMongoRepository {
    pub dao: TodosMongoDAO,
}

#[async_trait]
impl ReadOnlyEntityRepo<TodoStates, String> for TodosMongoRepository {
    async fn fetch_one(&self, id: String) -> Result<Option<Entity<TodoStates, String>>, String> {
        self.dao
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|dbo| dbo.into()))
    }

    async fn fetch_all(&self) -> Result<Vec<Entity<TodoStates, String>>, String> {
        self.dao
            .fetch_all()
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(|dbo| dbo.into())
                    .collect()
            })
    }
}

impl CanGetId<String> for EntityDBO<TodoDboState, String> {
    fn id(&self) -> &String {
        &self.entity_id
    }
}

#[async_trait]
impl WriteOnlyEntityRepo<TodoStates, String> for TodosMongoRepository {
    async fn insert(&self, todo: Entity<TodoStates, String>) -> Result<String, String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = todo.into();

        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: Some(0),
            ..entity_dbo.clone()
        };

        self.dao.insert(sanitize_version).await
    }

    async fn update(&self, id: String, todo: Entity<TodoStates, String>) -> Result<String, String> {
        let entity_dbo: EntityDBO<TodoDboState, String> = todo.into();
        let sanitize_version: EntityDBO<TodoDboState, String> = EntityDBO {
            version: entity_dbo.clone().version.map(|old| old + 1),
            ..entity_dbo.clone()
        };

        self.dao.update(id, sanitize_version).await
    }

    async fn delete(&self, id: String) -> Result<String, String> {
        self.dao.delete(id).await
    }
}
