use axum::{extract::State, Json};
use crate::{validation::ValidatedJson, AppError};

use super::{data_transfer_objects::{CreateEvent, CreateEventResult, EventDetails}, event_manager::EventManager};

pub async fn create(
    State(event_manager): State<EventManager>,
    ValidatedJson(data): ValidatedJson<CreateEvent>,
) -> Result<Json<CreateEventResult>, AppError> {
    let result = event_manager.create_event(data).await?;
    Ok(Json(result))
}

pub async fn get(
    State(event_manager): State<EventManager>
) -> Result<Json<Vec<EventDetails>>, AppError>  {
    let event_details = event_manager.list().await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(event_details))
}
