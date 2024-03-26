use std::sync::Arc;

use chrono::Utc;
use mongodb::Client;

use crate::
    baskets::application::BasketService
;

use super::{
    constants::CURRENCY,
    data_models::Payment,
    data_transfer_objects::{CheckoutRequest, CheckoutResponse},
    payment_providers::{pay_mongo_provider::PayMongoProvider, PaymentProvider}, persistence::{MongoDbPaymentRepository, PaymentRepository},
};

#[derive(Clone)]
pub struct PaymentService {
    basket_service: BasketService,
    payment_provider: Arc<dyn PaymentProvider>,
    payment_repository: Arc<dyn PaymentRepository>,
}

impl PaymentService {
    pub fn new(
        basket_service: BasketService,
        provider_base_url: String,
        secret_base64: String,
        payment_method_types: Vec<String>,
        client: Client, 
        database: String
    ) -> Self {
        let payment_provider = Arc::new(PayMongoProvider::new(
            provider_base_url,
            secret_base64,
            payment_method_types,
        ));

        let payment_repository = Arc::new(
            MongoDbPaymentRepository::new(
                client,
                database,
        ));
        Self {
            basket_service,
            payment_provider,
            payment_repository,
        }
    }

    pub async fn create_checkout(
        &self,
        checkout_request: CheckoutRequest,
    ) -> anyhow::Result<CheckoutResponse> {

        tracing::info!("Getting Basekt {}", &checkout_request.basket_id);
        let basket = self
            .basket_service
            .get_valid_basket(&checkout_request.basket_id)
            .await?
            .ok_or(anyhow::anyhow!(
                "Basket not found: {:?}",
                &checkout_request.basket_id
            ))?;

        tracing::info!("Creating Checkout");
        let checkout_data = self.payment_provider.prepare_checkout(&basket).await?;

        let payment = Payment::new(
            basket.total_price,
            CURRENCY.to_string(),
            self.payment_provider.get_name(),
            "initial".to_string(),
            Utc::now(),
            "checkout".to_string(),
            Some(checkout_data.clone()),
        );

        self.payment_repository.save(&payment).await?;

        Ok(CheckoutResponse::new(
            checkout_data.checkout_id,
            checkout_data.checkout_url,
        ))
    }
}
