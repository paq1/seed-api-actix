use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::core::shared::copy_from::CopyFromId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityDBO<DATA, ID> {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id_mongo: Option<ObjectId>,
    pub version: Option<i32>,
    pub entity_id: ID,
    pub data: DATA,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventDBO<DATA, ID> {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id_mongo: Option<ObjectId>,
    pub version: Option<i32>,
    pub entity_id: ID,
    pub event_id: ID,
    pub data: DATA,
}


impl<DATA, ID> CopyFromId<ID> for EntityDBO<DATA, ID>
where
    DATA: Clone,
    ID: Clone
{
    fn copy_from_id(&self, id: ID) -> Self {
        Self {
            entity_id: id,
            ..self.clone()
        }
    }
}