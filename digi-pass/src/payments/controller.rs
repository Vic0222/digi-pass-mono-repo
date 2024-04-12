use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{app_state::AppState, validation::ValidatedJson, AppError};

use super::{data_transfer_objects::{maya_webhook::MayaWebhookRequest, CheckoutRequest, CheckoutResponse}, webhook_handlers};

pub async fn checkout(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<CheckoutRequest>,
) ->  Result<Json<CheckoutResponse>, AppError>  {
    let result = state.payment_service.create_checkout(data).await?;

    Ok(Json(result))
}

pub async fn maya_webhook(
    State(state): State<Arc<AppState>>,
    Json(webhook): Json<MayaWebhookRequest>,
) ->  Result<(), AppError>  {

    let result = webhook_handlers::handle_maya_checkout_webhook(webhook, &state.payment_service).await;
    if let Err(err) = result {
        tracing::error!("Error handling webhook: {:?}", err);
    }

    Ok(())
}
