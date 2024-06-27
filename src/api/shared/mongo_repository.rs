use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::core::shared::copy_from::CopyFromId;
use crate::core::shared::repository::{ReadOnlyRepository, WriteOnlyRepository};

pub struct MongoRepository<ENTITY>
where
    ENTITY: Send + Sync,
{
    collection: Collection<ENTITY>,
}

impl<ENTITY> MongoRepository<ENTITY>
where
    ENTITY: Send + Sync,
{
    pub async fn new(dbname: String, name: String) -> Self {
        let uri = "mongodb://localhost:27017".to_string();
        let client: Client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(dbname.as_str());
        let collection: Collection<ENTITY> = db.collection(name.as_str());
        Self { collection }
    }
}

#[async_trait]
impl<ENTITY> ReadOnlyRepository<ENTITY, String> for MongoRepository<ENTITY>
where
    ENTITY: DeserializeOwned + Send + Sync
{
    async fn fetch_one(&self, id: String) -> Result<Option<ENTITY>, String> {
        let filter = doc! {"id": id};
        self.collection
            .find_one(filter)
            .await
            .map_err(|err| format!("err : {err}"))
    }
}

#[async_trait]
impl<ENTITY> WriteOnlyRepository<ENTITY, String> for MongoRepository<ENTITY>
where
    ENTITY: Serialize + CopyFromId + Send + Sync,
{
    async fn insert(&self, entity: ENTITY, id: Option<String>) -> Result<String, String> {
        let id_metier = id.unwrap_or(Self::generate_id());
        self.collection
            .insert_one(entity.copy_from_id(id_metier.clone()))
            .await
            .map_err(|err| err.to_string())
            .map(|_| id_metier)
    }
}

trait IdGenerator {
    fn generate_id() -> String;
}

impl<ENTITY> IdGenerator for MongoRepository<ENTITY>
where
    ENTITY: Send + Sync {
    fn generate_id() -> String {
        "lol".to_string()
    }
}