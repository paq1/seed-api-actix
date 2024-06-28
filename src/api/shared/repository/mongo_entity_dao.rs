use crate::api::shared::repository::dbos::EntityDBO;
use crate::api::shared::repository::mongo_dao::MongoDAO;

pub type EntityMongoDAO<DATADBO> = MongoDAO<EntityDBO<DATADBO, String>>;