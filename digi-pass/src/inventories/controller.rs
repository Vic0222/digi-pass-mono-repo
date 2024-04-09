use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{app_state::AppState, validation::ValidatedJson, AppError};

use super::data_transfer_objects::{CreateInventoryBatch, GenerateInventory, GenerateInventoryResult, ReserveInventories, ReserveInventoriesResult};

pub async fn generate_async(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<GenerateInventory>,
) ->  Result<Json<GenerateInventoryResult>, AppError>  {
    let result = state.inventory_service.generate_async(data).await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(result))
}


pub async fn add_batch(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<CreateInventoryBatch>,
) -> Result<StatusCode, AppError> {
    state.inventory_service.add_batch(data).await?;
    Ok(StatusCode::CREATED)
}


pub async fn reserve_inventories(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<ReserveInventories>,
) -> Result<Json<ReserveInventoriesResult>, AppError> {
    let result = state.inventory_service.reserve_inventories(&data).await?;
    Ok(Json(result))
}
