use std::sync::Arc;
use axum::{extract::{Path, State}, response::IntoResponse, Json};
use reqwest::StatusCode;
use crate::app_state::AppState;

use super::errors::PassError;


pub async fn get(
    State(state): State<Arc<AppState>>,
    Path(order_transaction_item_inventory_id): Path<String>,
) -> impl IntoResponse  {
    let result = state.pass_service.get_pass(&state.order_service, order_transaction_item_inventory_id.clone()).await;

    match result {
        Ok(jwt_pass) => {
            if let Some(jwt_pass) = jwt_pass {
                (StatusCode::OK, Json(jwt_pass)).into_response()
            }else{
                (StatusCode::NOT_FOUND).into_response()
            }
        },
        Err(err) => PassError::from(err).into_response(),
    }
}
