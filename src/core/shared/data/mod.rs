use serde::{Deserialize, Serialize};

use crate::core::shared::copy_from::CopyFromId;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity<S, REF> {
    pub entity_id: REF,
    pub data: S,
}

impl<S> CopyFromId<String> for Entity<S, String>
where
    S: Clone
{
    fn copy_from_id(&self, id: String) -> Self {
        Self {
            entity_id: id,
            ..self.clone()
        }
    }
}