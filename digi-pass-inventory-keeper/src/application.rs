use crate::{constants::INVENTORY_STATUS_RESERVED, persistence::InventoryRepository};

pub struct InventoryKeeperService {
    inventory_repository: InventoryRepository
}

impl InventoryKeeperService {
    
    pub fn new(inventory_repository: InventoryRepository) -> Self {
        Self {
            inventory_repository
        }
    }

    pub async fn do_inventory_keeping(&self) -> anyhow::Result<()> {
        //get data
        let inventories = self.inventory_repository.get_inventories(INVENTORY_STATUS_RESERVED.to_string(), chrono::Utc::now()).await?;
        tracing::info!("inventory count {}", inventories.len());
        
        //proccess data

        //save data
        Ok(())
    }
    
}