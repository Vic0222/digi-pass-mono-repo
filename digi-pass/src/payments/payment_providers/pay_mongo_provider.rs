mod data_transfer_objects;
use async_trait::async_trait;

use crate::{baskets::data_transfer_objects::{Basket, BasketItem}, payments::data_models::CheckoutData};

use self::data_transfer_objects::checkout::{self, request::LineItem};

use super::PaymentProvider;

pub struct PayMongoProvider {
    base_url: String,
    secret_base64: String,
    payment_method_types: Vec<String>,
}


impl PayMongoProvider {
    pub fn new(base_url: String, secret_base64: String, payment_method_types: Vec<String>) -> Self {
        Self {
            base_url,
            secret_base64,
            payment_method_types,
        }
    }
}


#[async_trait]
impl PaymentProvider for PayMongoProvider {
    async fn prepare_checkout(&self, basket: &Basket) -> anyhow::Result<CheckoutData>{

        let client = reqwest::Client::new();
        let line_items:Vec<LineItem> = basket.basket_items.iter().filter_map(basekt_item_to_line_item).collect();
        let request = checkout::request::CheckoutRequest::new(line_items, self.payment_method_types.clone());

        let res = client.post(format!("{}/{}", &self.base_url, "v1/checkout_sessions"))
        .header("Content-Type", "application/json")
        .header("accept", "application/json")
        .header("authorization", format!("Basic {}", self.secret_base64))
            .json(&request)
            .send()
            .await?
            .json::<checkout::response::CheckoutResult>()
            .await?;

        Ok(CheckoutData::new(res.data.id, res.data.attributes.checkout_url))
    }
}

fn basekt_item_to_line_item(basket_item: &BasketItem) -> Option<checkout::request::LineItem> {
    if basket_item.basketed_inventories.len() == 0 {
        return None;
    }
    let line_item = LineItem::new(
        "PHP",
        basket_item.basketed_inventories[0].price,
        &basket_item.basketed_inventories[0].event_id,
        basket_item.basketed_inventories.len() as i32,
        ""
    );

    Some(line_item)
}


