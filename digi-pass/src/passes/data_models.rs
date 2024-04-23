use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pass {
    pub sub: String, //orderline inventory id
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub inventory_id: String,
    pub event_id: String,
    pub event_name: String,
}