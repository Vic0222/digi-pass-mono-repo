use axum::{extract::State, Json};

use crate::{validation::ValidatedJson, AppError};

use super::{application::PaymentService, data_transfer_objects::{maya_webhook::MayaWebhookRequest, CheckoutRequest, CheckoutResponse}, webhook_handlers};

pub async fn checkout(
    State(payment_service): State<PaymentService>,
    ValidatedJson(data): ValidatedJson<CheckoutRequest>,
) ->  Result<Json<CheckoutResponse>, AppError>  {
    let result = payment_service.create_checkout(data).await?;

    Ok(Json(result))
}

pub async fn maya_webhook(
    State(payment_service): State<PaymentService>,
    Json(webhook): Json<MayaWebhookRequest>,
) ->  Result<(), AppError>  {

    let result = webhook_handlers::handle_maya_checkout_webhook(webhook, payment_service).await;
    if let Err(err) = result {
        tracing::error!("Error handling webhook: {:?}", err);
    }

    Ok(())
}