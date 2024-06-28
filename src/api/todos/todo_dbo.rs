use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TodoDboState {
    TodoDbo(TodoDbo)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoDbo {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TodoDboEvent {
    TodoCreatedDbo(TodoCreatedDbo)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoCreatedDbo {
    by: String,
    at: String,
}
