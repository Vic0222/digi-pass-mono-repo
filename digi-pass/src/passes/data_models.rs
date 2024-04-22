use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pass {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub nbf: usize,
    pub inventory_id: String,
    pub event_id: String,
    pub event_name: String,
    pub event_description: String
}