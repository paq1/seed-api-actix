use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::core::shared::copy_from::CopyFromId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoDbo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id_mongo: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
}

impl CopyFromId for TodoDbo {
    fn copy_from_id(&self, id: String) -> Self {
        Self {
            id: Some(id),
            ..self.clone()
        }
    }
}
