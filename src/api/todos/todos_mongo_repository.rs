use crate::api::shared::mongo_repository::MongoRepository;
use crate::api::todos::todo_dbo::TodoDbo;

pub type TodosMongoRepository = MongoRepository<TodoDbo>;