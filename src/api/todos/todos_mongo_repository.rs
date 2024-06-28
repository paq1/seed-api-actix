use async_trait::async_trait;
use crate::api::shared::repository::dbos::EntityDBO;
use crate::api::todos::todo_dbo::TodoDbo;
use crate::api::todos::todos_mongo_dao::TodosMongoDAO;
use crate::core::shared::data::Entity;
use crate::core::shared::repositories::dao::{ReadOnlyDAO, WriteOnlyDAO};
use crate::core::todos::data::Todo;
use crate::core::todos::todos_repository::{TodosRepositoryReadOnly, TodosRepositoryWriteOnly};

pub struct TodosMongoRepository {
    pub dao: TodosMongoDAO,
}

impl From<TodoDbo> for Todo {
    fn from(value: TodoDbo) -> Self {
        Todo {
            name: value.name.clone()
        }
    }
}


impl From<Entity<Todo, String>> for EntityDBO<TodoDbo, String> {
    fn from(value: Entity<Todo, String>) -> Self {
        EntityDBO {
            id_mongo: None,
            version: None,
            entity_id: value.entity_id.clone(),
            data: value.data.into(),
        }
    }
}

impl From<Todo> for TodoDbo {
    fn from(value: Todo) -> Self {
        TodoDbo {
            name: value.name.clone()
        }
    }
}

#[async_trait]
impl TodosRepositoryReadOnly for TodosMongoRepository {
    async fn fetch_one(&self, id: String) -> Result<Option<Entity<Todo, String>>, String> {
        self.dao
            .fetch_one(id).await
            .map(|maybedata| maybedata.map(|x| Entity { entity_id: x.entity_id, data: x.data.into() }))
    }
}

#[async_trait]
impl TodosRepositoryWriteOnly for TodosMongoRepository {
    async fn insert_one(&self, todo: Entity<Todo, String>) -> Result<String, String> {
        let entity_dbo: EntityDBO<TodoDbo, String> = todo.into();
        // fixme mettre la version a 0

        self.dao.insert(entity_dbo).await
    }
}