use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum TodoStates {
    Todo { name: String }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TodoEvents {
    Created {
        by: String,
        at: String
    }
}
