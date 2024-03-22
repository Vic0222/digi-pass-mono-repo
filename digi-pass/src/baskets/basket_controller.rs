use axum::{extract::State, Json};

use crate::{validation::ValidatedJson, AppError};

use super::{basket_manager::BasketManager, data_transfer_objects::{CreateBasketRequest, CreateBasketResult}};

pub async fn create(
    State(basket_manager): State<BasketManager>,
    ValidatedJson(data): ValidatedJson<CreateBasketRequest>,
) -> Result<Json<CreateBasketResult>, AppError> {
    let result = basket_manager.create_basket(data).await?;
    Ok(Json(result))
}