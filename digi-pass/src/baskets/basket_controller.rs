use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{app_state::AppState, validation::ValidatedJson, AppError};

use super::data_transfer_objects::{CreateBasketRequest, CreateBasketResult};

pub async fn create(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<CreateBasketRequest>,
) -> Result<Json<CreateBasketResult>, AppError> {
    let result = state.basket_service.create_basket(data).await?;
    Ok(Json(result))
}