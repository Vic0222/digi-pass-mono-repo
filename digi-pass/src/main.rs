mod events;
mod inventories;
mod app_state;
mod validation;
pub mod helpers;

use std::env;
use dotenv::dotenv;

use axum::{
    http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use mongodb::Client;
use serde::Serialize;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{app_state::AppState, events::{event_manager::EventManager, event_repository::MongoDbEventRepository}, inventories::{inventory_manager::InventoryManager, inventory_repository::MongoDbInventoryRepository}};

use jwt_authorizer::{JwtAuthorizer, Validation};
use jwt_authorizer::{Authorizer, IntoLayer};

use serde_json::Value;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt().json().init();

    tracing::info!("loading env variables from file");
    dotenv().ok();

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

    

    let event_repository = Box::new(MongoDbEventRepository::new(client.clone(), database.clone()));
    let event_manager = EventManager::new(event_repository);

    let inventory_repository = Box::new(MongoDbInventoryRepository::new(client.clone(), database.clone()));
    let inventory_manager = InventoryManager::new(inventory_repository);
    let state = AppState {
        event_manager,
        inventory_manager,
    };

    // build our application with a route
    let app = Router::<AppState>::new()
        // `GET /` goes to `root`
        .route("/", get(index))
        .route(
            "/events",
            post(self::events::events_controller::create),
        )
        .route(
            "/events",
            get(self::events::events_controller::get),
        ).route(
            "/inventories/generate",
            post(self::inventories::inventories_controller::generate_async),
        )
        .route(
            "/inventories/batch",
            post(self::inventories::inventories_controller::add_batch),
        )
        .layer(jwt_auth.into_layer())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind");
    tracing::info!("listening on {}", listener.local_addr().expect("failed to get local address"));
    axum::serve(listener, app).await.expect("Failed to start server");
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

struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}