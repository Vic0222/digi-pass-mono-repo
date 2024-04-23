use std::sync::Arc;

use anyhow::anyhow;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::Client;

use crate::{baskets::data_transfer_objects::Basket, orders::data_models::{OrderTransaction, OrderTransactionItem, OrderTransactionItemInventory, OrderTransactionPayment}};

use super::{data_transfer_objects, persistence::{MongoDbOrderTransactionRepository, OrderTransactionRepository}};

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

    pub async fn get_order_transactions(&self, order_transaction_id: String) -> anyhow::Result<Option<data_transfer_objects::OrderTransaction>> {

        let object_id  = match ObjectId::parse_str(&order_transaction_id)  {
            Ok(object_id) => object_id,
            Err(_) => {
                tracing::error!("Failed to parse ObjectId");
                return Ok(None);
            },
        };
        
        let order_transaction = self.order_transaction_repository.get(object_id).await?;
        let order_transaction =if let Some(order_transaction) = order_transaction {
            order_transaction
        }else {
            return  Ok(None);
        };

        Ok(Some(mpa_order_transaction_to_dto(order_transaction)))
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

fn mpa_order_transaction_to_dto(order_transaction: OrderTransaction) -> data_transfer_objects::OrderTransaction {
    
    let dto = data_transfer_objects::OrderTransaction {
        id: order_transaction.id.to_hex(),
        order_id: order_transaction.order_id.to_hex(),
        r#type: order_transaction.r#type.clone(),
        basket_id: order_transaction.basket_id.clone(),
        items: order_transaction.items.iter().map(mpa_order_transaction_item_to_dto).collect(),
        payments: order_transaction.payments.iter().map(mpa_order_transaction_payment_to_dto).collect(),
        created_at: order_transaction.created_at,
    };

    dto
}

fn mpa_order_transaction_item_to_dto(order_transaction_item: &OrderTransactionItem) -> data_transfer_objects::OrderTransactionItem {
    data_transfer_objects::OrderTransactionItem {
        id: order_transaction_item.id.clone(),
        created_at: order_transaction_item.created_at,
        price: order_transaction_item.price,
        inventories: order_transaction_item.inventories.iter().map(mpa_order_transaction_item_inventory_to_dto).collect(),
    }
}

fn mpa_order_transaction_item_inventory_to_dto(order_transaction_item_inventory: &OrderTransactionItemInventory) -> data_transfer_objects::OrderTransactionItemInventory {
    data_transfer_objects::OrderTransactionItemInventory {
        id: order_transaction_item_inventory.id.clone(),
        inventory_id: order_transaction_item_inventory.inventory_id.to_hex(),
        event_id: order_transaction_item_inventory.event_id.to_hex(),
        name: order_transaction_item_inventory.name.clone(),
        created_at: order_transaction_item_inventory.created_at,
    }
}

fn mpa_order_transaction_payment_to_dto(order_transaction_payment: &OrderTransactionPayment) -> data_transfer_objects::OrderTransactionPayment {
    data_transfer_objects::OrderTransactionPayment {
        id: order_transaction_payment.id.clone(),
        payment_id: order_transaction_payment.payment_id.to_hex(),
        amount: order_transaction_payment.amount,
        currency: order_transaction_payment.currency.clone(),
        provider: order_transaction_payment.provider.clone(),
        status: order_transaction_payment.status.clone(),
        payment_type: order_transaction_payment.payment_type.clone(),
        created_at: order_transaction_payment.created_at,
    }
}