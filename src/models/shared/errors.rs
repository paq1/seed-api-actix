use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone)]
pub struct StandardHttpError {
    pub not_found: Error,
    pub internal_server_error: Error,
    pub unauthorized: Error,
}

impl StandardHttpError {
    pub fn new() -> Self {
        Self {
            not_found: Error::new("ressource not found".to_string(), "00NOTFO".to_string(), vec![], Some(404)),
            internal_server_error: Error::new("wip".to_string(), "00INTER".to_string(), vec![], Some(500)),
            unauthorized: Error::new("wip".to_string(), "00UNAUT".to_string(), vec![], Some(401)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Error {
    #[schema(example = "titre")]
    pub title: String,
    #[schema(example = "00EXAMPLE")]
    pub code: String,
    #[schema(example = "[]")]
    pub causes: Vec<Problem>,
    #[schema(example = "200")]
    pub status: Option<u16>,
}

impl Error {
    pub fn new(title: String, code: String, problems: Vec<Problem>, status: Option<u16>) -> Self {
        Self {
            title,
            code,
            causes: problems,
            status,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Problem {
    #[schema(example = "titre")]
    pub title: String,
    #[schema(example = "description")]
    pub description: String,
}