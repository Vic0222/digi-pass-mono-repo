mod events;
mod inventories;
mod app_state;
mod validation;
mod baskets;
mod payments;
mod orders;
mod passes;
pub mod error;
pub mod helpers;

use std::{env, sync::Arc};
use aws_sdk_secretsmanager::types::Filter;
use lambda_http::run;

use dotenv::dotenv;

use axum::{
    routing::{get, post}, Json, Router
};
use mongodb::Client;
use serde::Serialize;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use crate::{app_state::AppState, orders::application::OrderService, passes::application::PassService, payments::application::PaymentService};
use crate::inventories::application::InventoryService;
use crate::events::application::EventService;
use crate::baskets::application::BasketService;

use jwt_authorizer::{JwtAuthorizer, Validation};
use jwt_authorizer::{Authorizer, IntoLayer};

use serde_json::Value;

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

pub async fn load_secrets_from_ssm() -> anyhow::Result<()> {
    
    if !is_running_in_lambda() {
        tracing::info!("Not in lambda, not loading secrets");
        return Ok(())
    }
    tracing::debug!("Loading Secrets:");
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ssm::Client::new(&config);
   
   let resp = client.get_parameters_by_path()
    .path("/DigiPass")
    .with_decryption(true).send().await?;

    if resp.parameters.is_none() {
        tracing::info!("No parameters found");
        return Ok(());
    }

    
    for parameter in resp.parameters() {
        if let Some(name) = parameter.name() {
            tracing::debug!("parameter found: {:?}", name);
            let name = name.replace("/DigiPass/", "");
            if let Some(value) = parameter.value() {
                env::set_var(name, value);
            }
        }
    }
    Ok(())
}

fn is_running_in_lambda() -> bool {
    env::var("AWS_LAMBDA_FUNCTION_NAME").is_ok()
}


#[tokio::main]
async fn main() {

    println!("loading env variables from file");
    if dotenv().is_ok() {
        println!("Env variables from file successful");
    }
    if is_running_in_lambda() {
        lambda_http::tracing::init_default_subscriber();
    }else{
        tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact().init();
    }
    
    load_secrets().await.expect("Failed loading secrets");

    let issuer = env::var("JwtConfig__Issuer").expect("Jwt issuer not found");
    let audience = env::var("JwtConfig__Audience").expect("Jwt audience not found");
    let validation = Validation::new()
                    .iss(&[issuer.clone()])
                    .aud(&[audience])
                    .nbf(true)
                    .leeway(20);

    let jwt_auth: Authorizer<Value> = JwtAuthorizer::from_oidc(&issuer[..])
                      .validation(validation).build().await.expect("Failed creating jwt authorizer");


    let connection_string = env::var("MongoDbConfig__ConnectionString")
        .expect("MongoDb connection string not found.");
    let database = env::var("MongoDbConfig__Database")
        .expect("MongoDb database not found");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(connection_string)
        .await
        .expect("Failed creating mongodb client");

    let maya_base_url = env::var("Maya__BaseUrl")
    .expect("Maya base url not found.");

    let maya_secret_base64 = env::var("Maya__SecretKeyBase64")
    .expect("Maya secret key not found.");

    
    let event_service = EventService::new(client.clone(), database.clone());

    let inventory_service = InventoryService::new(client.clone(), database.clone(), event_service.clone());

    let basket_service = BasketService::new(client.clone(), database.clone(), inventory_service.clone(), event_service.clone());

    let payment_service = PaymentService::new(basket_service.clone(), maya_base_url, maya_secret_base64, client.clone(), database.clone() );
    
    let order_service = OrderService::new(client.clone(), database.clone());

    let pass_private_key = env::var("PassPrivateKey").expect("Pass private key not found").to_string();
    let pass_service = PassService::new(pass_private_key).expect("Failed creating PassService");

    let state = Arc::new(AppState {
        event_service,
        inventory_service,
        basket_service,
        payment_service,
        order_service,
        pass_service
    });

    // build our application with a route
    let app = Router::<Arc<AppState>>::new()
        // `GET /` goes to `root`
        .route(
            "/events",
            post(self::events::events_controller::create),
        )
        .route(
            "/events",
            get(self::events::events_controller::get),
        ).route(
            "/inventories/generate",
            post(self::inventories::controller::generate_async),
        )
        .route(
            "/inventories/batch",
            post(self::inventories::controller::add_batch),
        )
        .route(
            "/inventories/reserve",
            post(self::inventories::controller::reserve_inventories),
        )
        .route(
            "/baskets",
            post(self::baskets::basket_controller::create),
        )
        .route(
            "/baskets/purchase",
            post(self::baskets::basket_controller::post_purchase),
        )
        .route(
            "/payments/checkout",
            post(self::payments::controller::checkout),
        )
        .route(
            "/passes/:order_transaction_item_inventory_id",
            get(self::passes::passes_controller::get),
        )
        .layer(jwt_auth.into_layer())
        .route(
            "/payments/webhook/maya",
            post(self::payments::controller::maya_webhook),
        )
        .route("/", get(index))
        .route("/version", get(index))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);
    if is_running_in_lambda() {
        tracing::info!("Running in lambda, starting server");
        run(app).await.expect("Failed running in lambda");
    }else{
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind");
        tracing::info!("listening on {}", listener.local_addr().expect("failed to get local address"));
        axum::serve(listener, app).await.expect("Failed to start server");
    }
}

#[derive(Serialize)]
struct Version {
    major: i32,
    minor: i32,
    revision: i32,
    build: i32,
}

async fn index() -> Json<Version> {
    let version = Version {
        major: 0,
        minor: 0,
        revision: 0,
        build: 1,
    };
    Json(version)
}

