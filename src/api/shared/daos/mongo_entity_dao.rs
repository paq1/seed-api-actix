use crate::api::shared::daos::dbos::EntityDBO;
use crate::api::shared::daos::mongo_dao::MongoDAO;

pub type EntityMongoDAO<DATADBO> = MongoDAO<EntityDBO<DATADBO, String>>;