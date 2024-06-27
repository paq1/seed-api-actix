use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;
use crate::core::shared::copy_from::CopyFromId;
use crate::core::shared::repositories::repository::{ReadOnlyRepository, WriteOnlyRepository};

pub struct MongoRepository<DBO>
where
    DBO: Send + Sync,
{
    collection: Collection<DBO>,
}

impl<DBO> MongoRepository<DBO>
where
    DBO: Send + Sync,
{
    pub async fn new(dbname: String, name: String) -> Self {
        let uri = "mongodb://localhost:27017".to_string();
        let client: Client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(dbname.as_str());
        let collection: Collection<DBO> = db.collection(name.as_str());
        Self { collection }
    }
}

#[async_trait]
impl<DBO> ReadOnlyRepository<DBO, String> for MongoRepository<DBO>
where
    DBO: DeserializeOwned + Send + Sync
{
    async fn fetch_one(&self, id: String) -> Result<Option<DBO>, String> {
        let filter = doc! {"id": id};
        self.collection
            .find_one(filter)
            .await
            .map_err(|err| format!("err : {err}"))
    }
}

#[async_trait]
impl<DBO> WriteOnlyRepository<DBO, String> for MongoRepository<DBO>
where
    DBO: Serialize + CopyFromId<String> + Send + Sync,
{
    async fn insert(&self, entity: DBO) -> Result<String, String> {
        let id_metier = Self::generate_id();
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

impl<DBO> IdGenerator for MongoRepository<DBO>
where
    DBO: Send + Sync {
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
}