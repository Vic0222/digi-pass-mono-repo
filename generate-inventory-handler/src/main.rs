mod generate_inventory;
mod model;
mod oauth_client;

use std::{env, time::Duration};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{
    config::Region,
    meta::PKG_VERSION,
    types::{DeleteMessageBatchRequestEntry, Message},
    Client,
};
use lambda_runtime::{run, service_fn, LambdaEvent};
use aws_lambda_events::{event::sqs::SqsEventObj, sqs::{BatchItemFailure, SqsBatchResponse}};
use model::Event;
use tokio::time::sleep;
use tracing_subscriber::fmt;

use crate::generate_inventory::GenerateInventoryHandler;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let format = fmt::format().without_time().with_target(false).compact();

    tracing_subscriber::fmt().event_format(format).init();

    dotenvy::dotenv()?;

    

    if is_running_in_lambda() {
        run(service_fn(function_handler)).await.expect("failed to start LambdaRuntime");
        
    }else{
        local().await.expect("failed to start local server");
    }

    Ok(())
}

fn is_running_in_lambda() -> bool {
    env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok()
}

async fn get_generate_inventory_handler() -> anyhow::Result<GenerateInventoryHandler> {
    let connection_string = env::var("MongoDbConfig__ConnectionString")?;
    let database = env::var("MongoDbConfig__Database")?;

    // Create a new client and connect to the server
    let mongodb_client = mongodb::Client::with_uri_str(connection_string).await?;

    let digi_pass_base_url = env::var("DigiPassBaseUrl")?;

    let oauth_client = oauth_client::get_client()?;
    let generate_inventory_handler = GenerateInventoryHandler::new(mongodb_client, database, oauth_client, digi_pass_base_url);

    Ok(generate_inventory_handler)
}


async fn local() -> anyhow::Result<()> {
    let queue_url = env::var("AWS_SQS_QUEUE_URL").expect("AWS_SQS_QUEUE_URL must be set");
    let region = env::var("AWS_REGION").expect("AWS_REGION must be set");
    let region_provider = RegionProviderChain::first_try(Region::new(region));

    tracing::info!("SQS client version: {}", PKG_VERSION);
    tracing::info!("Queue Url: {}", queue_url);
    tracing::info!(
        "Region:             {}",
        region_provider.region().await.unwrap().as_ref()
    );

    tracing::info!("Configuring Client");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let generate_inventory_handler = get_generate_inventory_handler().await?;
    
    tracing::info!("Starting Loop");
    let sleep_time = 10000;
    loop {
        let result = receive_and_delete(&client, &queue_url, &generate_inventory_handler).await;
        //result.unwrap();
        if let Err(error) = result {
            tracing::error!("received and delete encountered an error: {:?}", error);
        }
        tracing::info!("Ended a loop and sleeping for {} miliseconds", sleep_time);
        sleep(Duration::from_millis(sleep_time)).await;
    }
}


async fn receive_and_delete(client: &Client, queue_url: &String, generate_inventory_handler: &GenerateInventoryHandler) -> anyhow::Result<()> {
    let rcv_message_output = client.receive_message().queue_url(queue_url).send().await?;
    tracing::info!("Messages from queue with url: {}", queue_url);

    let mut delete_messages_builder = client.delete_message_batch().queue_url(queue_url);

    tracing::info!("Looping through messages.");

    for message in rcv_message_output.messages.unwrap_or_default() {
        //message.body.ok_or("No message body")?;
        tracing::info!("Processing a message with id: {:#?}", message.message_id);
        if let Some(body) = &message.body {
            let value = serde_json::from_str(body)?;
            let result = generate_inventory_handler.handle(&value).await;
            if let Err(err) = result {
                tracing::error!("Error handling message: Error {}",  err);
            }else{
                delete_messages_builder = delete_messages_builder.entries(get_delete_entry(&message)?);
            }
        }else{
            tracing::error!("No message body");
        }
        
    }

    if delete_messages_builder
        .get_entries()
        .as_ref()
        .is_some_and(|e| e.is_empty())
    {
        delete_messages_builder.send().await?;
    }
    Ok(())
}

fn get_delete_entry(message: &Message) -> anyhow::Result<DeleteMessageBatchRequestEntry> {
    let and_tupple = (message.message_id.clone(), message.receipt_handle.clone());
    match and_tupple {
        (Some(id), Some(receipt_handle)) => {
            let delete_entry = DeleteMessageBatchRequestEntry::builder()
                .id(id)
                .receipt_handle(receipt_handle)
                .build()?;
            Ok(delete_entry)
        }
        _ => Err(anyhow::anyhow!("No message id or receipt handle")),
    }
}

/// This is the main body for the function.
/// You can use the data sent into SQS here.
async fn function_handler(event: LambdaEvent<SqsEventObj<Event>>) -> anyhow::Result<SqsBatchResponse> {
    let handler = get_generate_inventory_handler().await?;
    
    let mut failures: Vec<BatchItemFailure> = vec![];
    for record in event.payload.records {
        let result = handler.handle(&record.body).await;
        
        if let Err(_) = result {
            if let Some(message_id) = record.message_id {
                failures.push(BatchItemFailure { item_identifier: message_id })
            }
            
        }
    }
    Ok(SqsBatchResponse {
        batch_item_failures: failures,
    })

}