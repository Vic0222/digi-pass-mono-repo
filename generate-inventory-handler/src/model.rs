
use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    pub version: String,

    pub id: String,

    pub detail_type: String,

    pub source: String,

    pub account: String,

    pub time: String,

    pub region: String,

    pub resources: Vec<String>,

    pub detail: Detail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(rename = "_id")]
    pub id: Id,

    pub operation_type: String,

    pub cluster_time: ClusterTime,

    pub wall_time: String,

    pub full_document: FullDocument,

    pub ns: Ns,

    pub document_key: DocumentKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ClusterTime {
    pub t: i64,

    pub i: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentKey {
    #[serde(rename = "_id")]
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullDocument {
    #[serde(rename = "_id")]
    pub id: String,

    pub event_id: String,

    pub number_to_create: i64,

    pub status: String,

    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "_data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ns {
    pub db: String,

    pub coll: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub event_id: ObjectId,
    pub status: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateInventory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub event_id: ObjectId,
    pub number_to_create: i32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInventoryBatch {
    pub event_id: String,
    pub quantity: i32,

    pub generate_inventory_id: Option<String>,
}