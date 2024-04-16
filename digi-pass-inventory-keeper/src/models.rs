use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    #[serde(rename = "_id", )]
    pub id: ObjectId,
    pub status: String,
    pub concurrency_stamp: String,
}