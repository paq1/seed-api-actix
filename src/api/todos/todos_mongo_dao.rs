use crate::api::shared::daos::mongo_entity_dao::EntityMongoDAO;
use crate::api::todos::todo_dbo::TodoDbo;

pub type TodosMongoDAO = EntityMongoDAO<TodoDbo>;