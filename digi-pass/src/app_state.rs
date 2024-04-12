use crate::{baskets::application::BasketService, events::application::EventService, inventories::application::InventoryService, payments::application::PaymentService};

#[derive(Clone)]
pub struct AppState {
    pub event_service: EventService,
    pub inventory_service: InventoryService,
    pub basket_service: BasketService,
    pub payment_service: PaymentService,
}
