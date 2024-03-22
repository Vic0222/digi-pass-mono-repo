use axum::extract::FromRef;

use crate::{baskets::basket_manager::BasketManager, events::event_manager::EventManager, inventories::inventory_manager::InventoryManager};
#[derive(FromRef,Clone)]
pub struct AppState {
    pub event_manager: EventManager,
    pub inventory_manager: InventoryManager,
    pub basket_manager: BasketManager,
}
