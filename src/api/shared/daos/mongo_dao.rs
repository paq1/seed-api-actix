use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::shared::can_get_id::CanGetId;
use crate::core::shared::daos::{ReadOnlyDAO, WriteOnlyDAO};

pub struct MongoDAO<DBO>
where
    DBO: Send + Sync,
{
    collection: Collection<DBO>,
}

impl<DBO> MongoDAO<DBO>
where
    DBO: Send + Sync,
{
    pub async fn new(dbname: String, name: String) -> Self {
        let uri = std::env::var("MONGO_URI").unwrap();
        let client: Client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(dbname.as_str());
        let collection: Collection<DBO> = db.collection(name.as_str());
        Self { collection }
    }
}

#[async_trait]
impl<DBO> ReadOnlyDAO<DBO, String> for MongoDAO<DBO>
where
    DBO: DeserializeOwned + Send + Sync,
{
    async fn fetch_one(&self, id: String) -> Result<Option<DBO>, String> {
        let filter = doc! {"id": id};
        self.collection
            .find_one(filter)
            .await
            .map_err(|err| format!("err : {err}"))
    }

    async fn fetch_all(&self) -> Result<Vec<DBO>, String> {
        self.find_all().await
            .map_err(|err| err.to_string())
    }
}

#[async_trait]
impl<DBO> WriteOnlyDAO<DBO, String> for MongoDAO<DBO>
where
    DBO: CanGetId<String> + Serialize
,
{
    async fn insert(&self, entity: DBO) -> Result<String, String> {
        self.collection
            .insert_one(entity.clone())
            .await
            .map_err(|err| err.to_string())
            .map(|_| entity.id().clone())
    }

    async fn update(&self, id: String, entity: DBO) -> Result<String, String> {
        let filter =  doc! { "id": id.clone() };
        self.collection
            .replace_one(filter, entity)
            .await
            .map(|_| id.clone())
            .map_err(|err| err.to_string())
    }

    async fn delete(&self, id: String) -> Result<String, String> {
        let filter =  doc! { "id": id.clone() };
        self.collection.delete_one(filter).await.map(|_| id).map_err(|err| err.to_string())
    }
}


impl<DBO> MongoDAO<DBO>
where
    DBO: DeserializeOwned + Send + Sync,
{
    async fn find_all(&self) -> Result<Vec<DBO>, mongodb::error::Error> {
        Ok(
            self.collection
                .find(doc! {})
                .await?
                .try_collect::<Vec<DBO>>()
                .await.unwrap()
        )
    }
}