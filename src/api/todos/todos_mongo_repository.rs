use std::fmt::format;
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use crate::api::todos::todo_dbo::TodoDbo;
use crate::core::todos::todos_repository::TodosRepository;

pub struct TodosMongoRepository {
    collection: Collection<TodoDbo>
}

impl TodosMongoRepository {
    pub async fn new() -> Self {
        let uri = "mongodb://localhost:27017".to_string();
        let client: Client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("tododbseed");
        let collection: Collection<TodoDbo> = db.collection("todos_store");
        TodosMongoRepository { collection }
    }
}

#[async_trait]
impl TodosRepository<TodoDbo, String> for TodosMongoRepository {
    async fn fetchOne(&self, id: String) -> Result<Option<TodoDbo>, String> {
        let filter = doc! {"id": id};
        self.collection
            .find_one(filter)
            .await
            .map_err(|err| format!("err : {err}"))
    }

    async fn insert(&self, entity: TodoDbo) -> Result<String, String> {
        let id_metier = "xxx".to_string();
        self.collection
            .insert_one(entity)
            .await
            .map_err(|err| String::from("xxx"))
            .map(|res| id_metier)
    }
}