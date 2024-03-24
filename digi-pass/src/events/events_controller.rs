use axum::{extract::State, Json};
use crate::{validation::ValidatedJson, AppError};

use super::{application::EventService, data_transfer_objects::{CreateEvent, CreateEventResult, EventDetails}};

pub async fn create(
    State(event_service): State<EventService>,
    ValidatedJson(data): ValidatedJson<CreateEvent>,
) -> Result<Json<CreateEventResult>, AppError> {
    let result = event_service.create_event(data).await?;
    Ok(Json(result))
}

pub async fn get(
    State(event_service): State<EventService>
) -> Result<Json<Vec<EventDetails>>, AppError>  {
    let event_details = event_service.list().await?;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(event_details))
}
