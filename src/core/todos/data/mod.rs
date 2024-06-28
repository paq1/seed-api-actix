use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum TodoStates {
    Todo(Todo)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub name: String
}
