use axum::{extract::State, Json};

use crate::{validation::ValidatedJson, AppError};

use super::{application::PaymentService, data_transfer_objects::{CheckoutRequest, CheckoutResponse}};

pub async fn checkout(
    State(payment_service): State<PaymentService>,
    ValidatedJson(data): ValidatedJson<CheckoutRequest>,
) ->  Result<Json<CheckoutResponse>, AppError>  {
    let result = payment_service.create_checkout(data).await?;

    Ok(Json(result))
}