use std::sync::Arc;

use anyhow::anyhow;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::Client;

use crate::{baskets::data_transfer_objects::Basket, orders::data_models::{OrderTransaction, OrderTransactionItem, OrderTransactionItemInventory, OrderTransactionPayment}};

use super::persistence::{MongoDbOrderTransactionRepository, OrderTransactionRepository};

pub struct  OrderService {
    order_transaction_repository: Arc<dyn OrderTransactionRepository>,
}



impl OrderService {
    
    pub fn new(client: Client, database: String) -> Self {
        let order_transaction_repository:Arc<dyn OrderTransactionRepository> = Arc::new(MongoDbOrderTransactionRepository::new(client, database));
        Self {
            order_transaction_repository
        }
    }

    pub async fn create_order(&self, basket: Basket) -> anyhow::Result<String> {
        let order_id = match basket.original_order_id {
            Some(id) => ObjectId::parse_str(id).map_err(|_| anyhow!("Invalid Original Order Id"))?,
            None => ObjectId::new(),
        }; 
        let order_transaction_id = ObjectId::new();

        let mut items = vec![];
        let mut index = 1;
        for basket_item in basket.basket_items {
            let order_transaction_item_id = format!("{}-{}", order_transaction_id, index);
            let mut inventories = vec![];
            let mut order_transaction_item_inventory_index = 1; 
            for basketed_inventory in basket_item.basketed_inventories {
                let inventory = OrderTransactionItemInventory {
                    id: format!("{}-{}", order_transaction_item_id, order_transaction_item_inventory_index),
                    inventory_id: ObjectId::parse_str(basketed_inventory.inventory_id)?,
                    event_id: ObjectId::parse_str(basketed_inventory.event_id)?,
                    name: basketed_inventory.name,
                    created_at: Utc::now(),
                };

                inventories.push(inventory);
                order_transaction_item_inventory_index += 1;
            }

            let item = OrderTransactionItem {
                id: order_transaction_item_id,
                created_at: Utc::now(),
                price: basket_item.price,
                inventories
            };
            items.push(item);
            index = index + 1;
        }

        let mut payments = vec![];
        let mut payment_index = 1;
        for payment_view in basket.payments.iter() {
            let payment = OrderTransactionPayment {
                id: format!("{}-{}", order_transaction_id, index),
                payment_id: ObjectId::parse_str(&payment_view.id)?,
                amount: payment_view.amount,
                currency: payment_view.currency.clone(),
                provider: payment_view.provider.clone(),
                status: payment_view.status.clone(),
                payment_type: payment_view.payment_type.clone(),
                created_at: Utc::now(),
            };

            payments.push(payment);
            payment_index = payment_index + 1;
        }
        
        let order_transaction = OrderTransaction {
            id: order_transaction_id,
            order_id,
            r#type: "sale".to_string(),
            basket_id: Some(basket.id.to_string()),
            items,
            payments,
            created_at: Utc::now(),
        };
        
        self.order_transaction_repository.save(&order_transaction).await?;

        return Ok(order_id.to_hex());
    }
}