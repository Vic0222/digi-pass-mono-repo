mod data_transfer_objects;
use async_trait::async_trait;

use crate::{baskets::data_transfer_objects::{Basket, BasketItem}, payments::data_models::CheckoutData};


use self::data_transfer_objects::checkout::{self, request::Item};

use super::PaymentProvider;

pub struct MayaProvider {
    base_url: String,
    secret_base64: String
}


impl MayaProvider {
    pub fn new(base_url: String, secret_base64: String, payment_method_types: Vec<String>) -> Self {
        Self {
            base_url,
            secret_base64
        }
    }
}


#[async_trait]
impl PaymentProvider for MayaProvider {
    fn get_name(&self) -> String {
        "Maya".to_string()
    }
    
    async fn prepare_checkout(&self, basket: &Basket) -> anyhow::Result<CheckoutData>{

        let client = reqwest::Client::new();
        let items:Vec<Item> = basket.basket_items.iter().filter_map(basket_item_to_item).collect();
        let request = checkout::request::CheckoutRequest::new(
            basket.total_price as f64 / 100.0,
            "PHP".to_string(),
            basket.id.to_string(),
            items,
        );

        let response  = client.post(format!("{}/{}", &self.base_url, "v1/checkout_sessions"))
        .header("Content-Type", "application/json")
        .header("accept", "application/json")
        .header("authorization", format!("Basic {}", self.secret_base64))
            .json(&request)
            .send()
            .await?;

        
        match response.error_for_status_ref() {
            Ok(_) => {
                let data = response.json::<checkout::response::CheckoutResult>().await?;
                Ok(CheckoutData::new(data.data.id, data.data.attributes.checkout_url))
            },
            Err(err) => {
                tracing::error!("Response Text: {:?}", response.text().await?);
                tracing::error!("Error: {:?}", err);
                Err(anyhow::anyhow!("Something went wrong generating checkout data."))
            }
        }
        
    }
}

fn basket_item_to_item(basket_item: &BasketItem) -> Option<checkout::request::Item> {
    if basket_item.basketed_inventories.is_empty() {
        return None;
    }
    let line_item = Item::new(
        basket_item.basketed_inventories[0].price as f64 / 100.00,
        basket_item.basketed_inventories.len().to_string(),
        basket_item.total_price as f64 / 100.00,
        basket_item.basketed_inventories[0].name.to_string(),
        basket_item.basketed_inventories[0].event_id.to_string(),
        basket_item.basketed_inventories[0].name.to_string(),
    );

    Some(line_item)
}


