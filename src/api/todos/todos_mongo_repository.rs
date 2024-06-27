use crate::api::shared::repository::EntityMongoRepo;
use crate::api::todos::todo_dbo::TodoDbo;

pub type TodosMongoRepository = EntityMongoRepo<TodoDbo>;