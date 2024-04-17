use bson::oid::ObjectId;

use crate::{constants::INVENTORY_STATUS_RESERVED, persistence::{InventoryRepository, OrderTransactionRepository}};

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
        let inventories = self.inventory_repository.get_inventories(INVENTORY_STATUS_RESERVED.to_string(), chrono::Utc::now()).await?;
        tracing::info!("inventory count {}", inventories.len());
        
        let inventory_ids: Vec<ObjectId> = inventories.iter().map(|i| i.id).collect();
        
        //proccess data
        let order_transactions = self.order_transaction_repostiry.get_order_tranactions(inventory_ids).await?;

        tracing::info!("orders count {}", order_transactions.len());
        //save data
        Ok(())
    }
    
}