use std::{str::FromStr, sync::Arc};

use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::Client;

use crate::{
    baskets::application::BasketService, payments::constants::{PAYMENT_STATUS_PENDING, PAYMENT_TYPE_CHECKOUT}}
;

use super::{
    constants::{CURRENCY, PAYMENT_STATUS_PAID},
    data_models::Payment,
    data_transfer_objects::{CheckoutRequest, CheckoutResponse, PaymentView},
    payment_providers::{maya_provider::MayaProvider, PaymentProvider}, persistence::{MongoDbPaymentRepository, PaymentRepository},
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
        client: Client, 
        database: String
    ) -> Self {
        let payment_provider = Arc::new(MayaProvider::new(
            provider_base_url,
            secret_base64,
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
            Some(ObjectId::from_str(&basket.id)?),
            basket.price,
            CURRENCY.to_string(),
            self.payment_provider.get_name(),
            PAYMENT_STATUS_PENDING.to_string(),
            Utc::now(),
            PAYMENT_TYPE_CHECKOUT.to_string(),
            Some(checkout_data.clone()),
        );

        self.payment_repository.save(&payment).await?;

        Ok(CheckoutResponse::new(
            checkout_data.checkout_id,
            checkout_data.checkout_url,
        ))
    }

    pub async fn mark_payment_as_paid(&self, checkout_id: &str) -> anyhow::Result<()> {
        
        let mut payment = self
            .payment_repository
            .find_one_by_checkout_id(checkout_id)
            .await?
            .ok_or(anyhow::anyhow!("Payment not found: checkout_id:{:?}", checkout_id))?;
        
        payment.status = PAYMENT_STATUS_PAID.to_string();

        self.payment_repository.update(&mut payment).await?;
        Ok(())
    }
    
    pub async fn get_basket_payments(&self, basket_id: &str) -> anyhow::Result<Vec<PaymentView>> {
        let payments = self.payment_repository.find_by_basket_id(basket_id).await?;
        let payment_views = payments.iter().map(map_payment_to_payment_view).collect();

        return Ok(payment_views);
    }
}

fn map_payment_to_payment_view(payment: &Payment) -> PaymentView {
    let id = payment.id.ok_or(anyhow::anyhow!("Failed to get object id for payment id")).unwrap().to_hex();
    let basket_id = payment.basket_id.ok_or(anyhow::anyhow!("Failed to get object id for basket id")).unwrap().to_hex();
    PaymentView {
        id: id,
        status: payment.status.clone(),
        created_at: payment.created_at,
        amount: payment.amount,
        currency: payment.currency.clone(),
        payment_type: payment.payment_type.clone(),
        basket_id: basket_id,
        concurrency_stamp: payment.concurrency_stamp.clone(),
        provider: payment.provider.clone(),
    }
}
