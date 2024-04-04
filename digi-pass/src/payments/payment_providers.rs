pub mod maya_provider;
use async_trait::async_trait;
use crate::baskets::data_transfer_objects::Basket;

use super::data_models::CheckoutData;

#[async_trait]
pub trait PaymentProvider : Send + Sync {
    fn get_name(&self) -> String;
    async fn prepare_checkout(&self, basket: &Basket) -> anyhow::Result<CheckoutData>;
}
