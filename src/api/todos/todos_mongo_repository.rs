use async_trait::async_trait;

use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::todos::todo_dbo::{TodoDbo, TodoDboState};
use crate::api::todos::todos_mongo_dao::TodosMongoDAO;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::shared::data::Entity;
use crate::core::todos::data::{Todo, TodoStates};
use crate::core::todos::todos_repository::{TodosRepositoryReadOnly, TodosRepositoryWriteOnly};

pub struct TodosMongoRepository {
    pub dao: TodosMongoDAO,
}

impl From<TodoDboState> for TodoStates {
    fn from(value: TodoDboState) -> Self {
        match value {
            TodoDboState::TodoDbo(data) => TodoStates::Todo(Todo {
                name: data.name.clone()
            })
        }
    }
}


impl From<Entity<TodoStates, String>> for EntityDBO<TodoDboState, String> {
    fn from(value: Entity<TodoStates, String>) -> Self {
        EntityDBO {
            id_mongo: None,
            version: None,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
        }
    }
}

impl From<TodoStates> for TodoDboState {
    fn from(value: TodoStates) -> Self {
        match value {
            TodoStates::Todo(x) => TodoDboState::TodoDbo(TodoDbo { name: x.name.clone() })
        }
    }
}

#[async_trait]
impl TodosRepositoryReadOnly for TodosMongoRepository {
    async fn fetch_one(&self, id: String) -> Result<Option<Entity<TodoStates, String>>, String> {
        self.dao
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|x| Entity { entity_id: x.entity_id, data: x.data.into() }))
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
