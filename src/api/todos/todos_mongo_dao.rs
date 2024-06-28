use crate::api::shared::repository::mongo_entity_dao::EntityMongoDAO;
use crate::api::todos::todo_dbo::TodoDbo;

pub type TodosMongoDAO = EntityMongoDAO<TodoDbo>;