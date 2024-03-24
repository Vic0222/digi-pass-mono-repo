use axum::{extract::State, Json};

use crate::{validation::ValidatedJson, AppError};

use super::{application::BasketService, data_transfer_objects::{CreateBasketRequest, CreateBasketResult}};

pub async fn create(
    State(basket_service): State<BasketService>,
    ValidatedJson(data): ValidatedJson<CreateBasketRequest>,
) -> Result<Json<CreateBasketResult>, AppError> {
    let result = basket_service.create_basket(data).await?;
    Ok(Json(result))
}