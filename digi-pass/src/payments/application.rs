use crate::{baskets::application::BasketService, events::application::EventService, inventories::application::InventoryService};

use super::{data_transfer_objects::{CheckoutRequest, CheckoutResponse}, payment_providers::{pay_mongo_provider::PayMongoProvider, PaymentProvider}};

pub struct PaymentService {
    basket_service: BasketService,
    inventory_service: InventoryService,
    event_service: EventService,
    payment_provider: Box<dyn PaymentProvider>,
}

impl PaymentService {
    pub fn new(basket_service: BasketService, inventory_service: InventoryService, event_service: EventService, provider_base_url: String, secret_base64: String, payment_method_types: Vec<String>) -> Self {
        let payment_provider = Box::new(PayMongoProvider::new(provider_base_url, secret_base64, payment_method_types));
        Self {basket_service, inventory_service, event_service, payment_provider}
    }

    pub async fn create_checkout(&self, checkout_request: CheckoutRequest) -> anyhow::Result<CheckoutResponse> {
        let basket = self.basket_service.get_valid_basket(&checkout_request.basket_id).await?
            .ok_or(anyhow::anyhow!("Basket not found: {:?}", &checkout_request.basket_id))?;

        let checkout_data = self.payment_provider.prepare_checkout(&basket).await?;

        Ok(CheckoutResponse::new(checkout_data.checkout_id, checkout_data.checkout_url))
    }
}

