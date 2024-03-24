pub mod pay_mongo_provider;
use async_trait::async_trait;

use crate::baskets::data_transfer_objects::Basket;

use super::data_models::CheckoutData;

#[async_trait]
pub trait PaymentProvider {
    async fn prepare_checkout(&self, basket: &Basket) -> anyhow::Result<CheckoutData>;
}