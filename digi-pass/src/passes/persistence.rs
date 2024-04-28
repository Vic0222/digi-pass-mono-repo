use bson::{doc, oid::ObjectId};
use mongodb::{options::FindOneOptions, Client, Collection};

use super::data_models::PassVerification;

#[async_trait::async_trait]
pub trait PassVerificationRepository : Send + Sync{
    async fn save_pass_verification(&self, pass_verification: &PassVerification) -> anyhow::Result<()>;
    async fn get_last_pass_verification(&self, inventory_id: ObjectId) -> anyhow::Result<Option<PassVerification>>;
}


pub struct MongoDbPassVerificationRepository{
    pub client: Client,
    pub database: String,
    pub collection: String,
}

impl MongoDbPassVerificationRepository {
    pub fn new(client: Client, database: String) -> Self {
        MongoDbPassVerificationRepository {
            client,
            database,
            collection: "PassVerifications".to_string(),
        }
    }

    fn get_collection(&self) -> Collection<PassVerification> {
        let database = self.client.database(&self.database[..]);
        database.collection::<PassVerification>(&self.collection[..])
    }
}

#[async_trait::async_trait]
impl PassVerificationRepository for MongoDbPassVerificationRepository {
    async fn save_pass_verification(&self, pass_verfication: &PassVerification) -> anyhow::Result<()>{
        let collection = self.get_collection();
        let _ = collection.insert_one(pass_verfication, None).await?;
        Ok(())
    }

     async fn get_last_pass_verification(&self, inventory_id: ObjectId) -> anyhow::Result<Option<PassVerification>> {
        let collection = self.get_collection();
        let options = FindOneOptions::builder().sort(doc! {
            "verification_time" : -1
        }).build();
        let pass_verification = collection.find_one(doc! { "inventory_id": inventory_id }, options).await?;

        return Ok(pass_verification);
     }
}