use crate::api::shared::repository::dbos::Entity;
use crate::api::shared::repository::mongo_repository::MongoRepository;

pub mod mongo_repository;
pub mod dbos;

pub type EntityMongoRepo<DATA> = MongoRepository<Entity<DATA, String>>;