use axum::{extract::State, http::HeaderMap, Json};

use crate::{app_state::AppState, validation::ValidatedJson, AppError};

use super::{application::PaymentService, data_transfer_objects::{webhook::Webhook, CheckoutRequest, CheckoutResponse}, webhook_handlers};

pub async fn checkout(
    State(payment_service): State<PaymentService>,
    ValidatedJson(data): ValidatedJson<CheckoutRequest>,
) ->  Result<Json<CheckoutResponse>, AppError>  {
    let result = payment_service.create_checkout(data).await?;

    Ok(Json(result))
}

pub async fn paymongo_webhook(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    body: String,
) ->  Result<(), AppError>  {
    let webhook: Webhook = serde_json::from_slice(body.as_bytes())?;
    let raw_signature = headers.get("Paymongo-Signature").ok_or(anyhow::anyhow!("Missing Paymongo-Signature"))?.to_str()?;
    let result = webhook_handlers::handle_checkout_webhook(webhook, app_state.pay_mongo_checkout_webhook_key.as_str(), body, raw_signature, app_state.payment_service).await;
    if let Err(err) = result {
        tracing::error!("Error handling webhook: {:?}", err);
    }

    Ok(())
}