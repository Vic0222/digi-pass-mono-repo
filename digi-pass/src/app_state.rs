use crate::{baskets::application::BasketService, events::application::EventService, inventories::application::InventoryService, orders::application::OrderService, passes::application::PassService, payments::application::PaymentService};

pub struct AppState {
    pub event_service: EventService,
    pub inventory_service: InventoryService,
    pub basket_service: BasketService,
    pub payment_service: PaymentService,
    pub order_service: OrderService,
    pub pass_service: PassService,
}
