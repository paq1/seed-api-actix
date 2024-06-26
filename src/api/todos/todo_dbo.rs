use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDbo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id_mongo: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
}
