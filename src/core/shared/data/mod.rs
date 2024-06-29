use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity<S, REF> {
    pub entity_id: REF,
    pub data: S,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntityEvent<E, REF> {
    pub entity_id: REF,
    pub data: E,
    pub event_id: REF
}
