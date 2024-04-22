use std::sync::Arc;
use crate::error::AppError;
use axum::{extract::{Path, State}, Json};
use crate::app_state::AppState;

use super::data_transfer_objects::JwtPass;


pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(order_transaction_item_inventory_id): Path<String>,
) -> Result<Json<JwtPass>, AppError>  {
    let jwt_pass = state.pass_service.get_pass("".to_string()).await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(jwt_pass))
}
