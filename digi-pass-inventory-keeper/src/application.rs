use bson::oid::ObjectId;

use crate::{constants, models::{Inventory, InventoryUpdate, OrderTransaction}, persistence::{InventoryRepository, OrderTransactionRepository}};

pub struct InventoryKeeperService {
    inventory_repository: InventoryRepository,
    order_transaction_repostiry: OrderTransactionRepository
}

impl InventoryKeeperService {
    
    pub fn new(inventory_repository: InventoryRepository, order_transaction_repostiry: OrderTransactionRepository) -> Self {
        InventoryKeeperService {
            inventory_repository,
            order_transaction_repostiry
        }
    }

    pub async fn do_inventory_keeping(&self) -> anyhow::Result<()> {
        //get data
        let inventories = self.inventory_repository.get_inventories(constants::INVENTORY_STATUS_RESERVED.to_string(), chrono::Utc::now()).await?;
        tracing::info!("inventory count {}", inventories.len());
        
        let inventory_ids: Vec<ObjectId> = inventories.iter().map(|i| i.id).collect();
        
        //proccess data
        let order_transactions = self.order_transaction_repostiry.get_order_tranactions(inventory_ids).await?;
        
        tracing::info!("orders count {}", order_transactions.len());

        let inventory_updates = get_inventory_updates(inventories, &order_transactions);
        tracing::info!("inventory_updates count {}", inventory_updates.len());
        //save data

        self.inventory_repository.update_inventory_status(inventory_updates).await?;
        Ok(())
    }
    
}

fn get_inventory_updates(inventories: Vec<crate::models::Inventory>, order_transactions: &Vec<OrderTransaction>) -> Vec<InventoryUpdate> {
    inventories.iter().map(|inventory|create_inventory_update(inventory, order_transactions)).collect()
}

fn create_inventory_update(inventory: &Inventory, order_transactions: &Vec<OrderTransaction>) -> InventoryUpdate {
    let status:String;

    if order_contains_inventory(order_transactions, &inventory.id) {
        status = constants::INVENTORY_STATUS_SOLD.to_string();
    }
    else {
        status = constants::INVENTORY_STATUS_AVAILABLE.to_string();
    }
    
    InventoryUpdate {
        id: inventory.id,
        status: status,
        concurrency_stamp: inventory.concurrency_stamp.clone()
    }
}

fn order_contains_inventory(order_transactions: &Vec<OrderTransaction>, inventory_id: &ObjectId) -> bool {
    order_transactions.iter().any(|order_transaction|
        order_transaction.items.iter().any(|i| i.inventories.iter().any(|ii| &ii.inventory_id == inventory_id))
    )
}


#[cfg(test)]
mod tests {
    use crate::{constants::INVENTORY_STATUS_RESERVED, models::{OrderTransactionItem, OrderTransactionItemInventory}};

    use super::*;

    #[test]
    fn test_create_inventory_update_sold() {
        //arrange
        let inventory_id = ObjectId::new();
        let inventory = Inventory {
            id: inventory_id.clone(),
            concurrency_stamp: Default::default(),
            status: INVENTORY_STATUS_RESERVED.to_string(),
        };

        let order_transaction = OrderTransaction {
            items: vec![
                OrderTransactionItem {
                    inventories: vec![
                        OrderTransactionItemInventory{
                            inventory_id:inventory.id.clone(),
                            ..Default::default()
                        }],
                    ..Default::default()
                }],
            ..Default::default()
        };

        //act
        let inventory_update = create_inventory_update(&inventory, &vec![order_transaction]);
        //asert
        assert_eq!(inventory_update.status, super::constants::INVENTORY_STATUS_SOLD.to_string());
    }

    #[test]
    fn test_create_inventory_update_available() {
        //arrange
        let inventory_id = ObjectId::new();
        let inventory = Inventory {
            id: inventory_id.clone(),
            concurrency_stamp: Default::default(),
            status: INVENTORY_STATUS_RESERVED.to_string(),
        };

        let order_transaction = OrderTransaction {
            items: vec![
                OrderTransactionItem {
                    inventories: vec![
                        OrderTransactionItemInventory{
                            inventory_id:ObjectId::new(),
                            ..Default::default()
                        }],
                    ..Default::default()
                }],
            ..Default::default()
        };

        //act
        let inventory_update = create_inventory_update(&inventory, &vec![order_transaction]);
        
        //assert
        assert_eq!(inventory_update.status, super::constants::INVENTORY_STATUS_AVAILABLE.to_string());
    }
}
