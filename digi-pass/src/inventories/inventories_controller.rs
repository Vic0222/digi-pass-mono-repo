use axum::{extract::State, http::StatusCode, Json};

use crate::{validation::ValidatedJson, AppError};

use super::{data_transfer_objects::{CreateInventoryBatch, GenerateInventory, GenerateInventoryResult, ReserveInventories, ReserveInventoriesResult}, inventory_manager::InventoryManager};

pub async fn generate_async(
    State(inventory_manager): State<InventoryManager>,
    ValidatedJson(data): ValidatedJson<GenerateInventory>,
) ->  Result<Json<GenerateInventoryResult>, AppError>  {
    let result = inventory_manager.generate_async(data).await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(result))
}


pub async fn add_batch(
    State(inventory_manager): State<InventoryManager>,
    ValidatedJson(data): ValidatedJson<CreateInventoryBatch>,
) -> Result<StatusCode, AppError> {
    inventory_manager.add_batch(data).await?;
    Ok(StatusCode::CREATED)
}


pub async fn reserve_inventories(
    State(inventory_manager): State<InventoryManager>,
    ValidatedJson(data): ValidatedJson<ReserveInventories>,
) -> Result<Json<ReserveInventoriesResult>, AppError> {
    let result = inventory_manager.reserve_inventories(&data).await?;
    Ok(Json(result))
}
