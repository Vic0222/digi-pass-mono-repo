use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{app_state::AppState, validation::ValidatedJson, AppError};

use super::data_transfer_objects::{
    CreateBasketRequest, CreateBasketResult, PurchaseBasketRequest, PurchaseBasketResult,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<CreateBasketRequest>,
) -> Result<Json<CreateBasketResult>, AppError> {
    let result = state.basket_service.create_basket(data).await?;
    Ok(Json(result))
}

pub async fn post_purchase(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<PurchaseBasketRequest>,
) -> Result<Json<PurchaseBasketResult>, AppError> {
    let result = state.basket_service.purchase_basket(&state.payment_service, &state.order_service, &data.basket_id).await?;
    return  Ok(Json(PurchaseBasketResult { order_id: result }));
}
