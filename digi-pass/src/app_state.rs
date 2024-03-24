use axum::extract::FromRef;

use crate::{baskets::application::BasketService, events::application::EventService, inventories::application::InventoryService};
#[derive(FromRef,Clone)]
pub struct AppState {
    pub event_service: EventService,
    pub inventory_service: InventoryService,
    pub basket_service: BasketService,
}
