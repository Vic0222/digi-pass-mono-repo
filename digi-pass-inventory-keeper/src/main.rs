mod application;
mod models;
mod persistence;
mod constants;

use std::env;

use aws_sdk_secretsmanager::types::Filter;
use dotenvy::dotenv;
use mongodb::Client;
use tracing_subscriber::fmt;

use crate::{application::InventoryKeeperService, persistence::{InventoryRepository, OrderTransactionRepository}};

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    
    if is_running_in_lambda() {
        load_secrets().await.expect("failed to load secrets");
    }else{
        println!("loading env variables from file");
        if dotenv().is_ok() {
            println!("Env variables from file successful");
        }
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


fn is_running_in_lambda() -> bool {
    env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok()
}

pub async fn load_secrets() -> anyhow::Result<()> {
    
    if !is_running_in_lambda() {
        tracing::info!("Not in lambda, not loading secrets");
        return Ok(())
    }
    tracing::debug!("Loading Secrets:");
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_secretsmanager::Client::new(&config);
    let filter = Filter::builder().key("name".into()).values("DigiPass__").build();
    let resp = client.list_secrets().filters(filter).send().await?;
    let secrets = resp.secret_list();
    tracing::debug!("Loading Secrets 222: {:?}", secrets);
    for secret in secrets {
        
        tracing::debug!("Secret found: {:?}", secret.name());
        if let Some(name) = secret.name() {
            let resp = client.get_secret_value().secret_id(name).send().await?;
            let name = name.replace("DigiPass__", "");
            env::set_var(&name, resp.secret_string().ok_or(anyhow::anyhow!("Failed getting secret"))?);
        }
    }
    Ok(())
}