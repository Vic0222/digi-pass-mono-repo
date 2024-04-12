use std::sync::Arc;

use axum::{extract::State, Json};
use crate::{app_state::AppState, validation::ValidatedJson, AppError};

use super::data_transfer_objects::{CreateEvent, CreateEventResult, EventDetails};

pub async fn create(
    State(state): State<Arc<AppState>>,
    ValidatedJson(data): ValidatedJson<CreateEvent>,
) -> Result<Json<CreateEventResult>, AppError> {
    let result = state.event_service.create_event(data).await?;
    Ok(Json(result))
}

pub async fn get(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<EventDetails>>, AppError>  {
    let event_details = state.event_service.list().await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(event_details))
}
