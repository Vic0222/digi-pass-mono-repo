mod application;
mod models;
mod persistence;
mod constants;

use std::env;

use dotenvy::dotenv;
use mongodb::Client;
use tracing_subscriber::fmt;

use crate::{application::InventoryKeeperService, persistence::{InventoryRepository, OrderTransactionRepository}};

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    println!("loading env variables from file");
    if dotenv().is_ok() {
        println!("Env variables from file successful");
    }
    let connection_string = env::var("MongoDbConfig__ConnectionString")
        .expect("MongoDb connection string not found.");
    let database = env::var("MongoDbConfig__Database")
        .expect("MongoDb database not found");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(connection_string)
        .await
        .expect("Failed creating mongodb client");
    
    let format = fmt::format().without_time().with_target(false).compact();
    
    tracing_subscriber::fmt().event_format(format).init();
    tracing::info!("Starting Inventory Keeper!");
    let inventory_repository = InventoryRepository::new(client.clone(), database.clone(), "Inventories".to_string());
    let order_transaction_repository = OrderTransactionRepository::new(client.clone(), database.clone(), "OrderTransactions".to_string());
    let keeper_service = InventoryKeeperService::new(inventory_repository, order_transaction_repository);

    //we probably want to add a retry mechanism here
    keeper_service.do_inventory_keeping().await.unwrap();

    Ok(())
}
