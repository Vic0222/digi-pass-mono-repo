use std::str::FromStr;

use anyhow::Ok;
use bson::oid::ObjectId;
use chrono::Utc;

use super::constants::{GENERATE_INVENTORY_STATUS_PENDING, INVENTORY_STATUS_AVAILABLE};
use super::data_models::{GenerateInventory, Inventory};
use super::data_transfer_objects::{
    CreateInventoryBatch, GenerateInventory as GenerateInventoryDto,
    GenerateInventoryResult,
};
use super::inventory_repository::InventoryRepository;

#[derive(Clone)]
pub struct InventoryManager {
    pub inventory_repository: Box<dyn InventoryRepository>,
}

impl InventoryManager {
    pub fn new(inventory_repository: Box<dyn InventoryRepository>) -> Self {
        InventoryManager {
            inventory_repository,
        }
    }

    pub async fn generate_async(
        &self,
        data: GenerateInventoryDto,
    ) -> anyhow::Result<GenerateInventoryResult> {
        let generate_inventory = map_generate_inventory(data)?;
        let result = self
            .inventory_repository
            .add_generate_inventory(&generate_inventory)
            .await?;
        Ok(GenerateInventoryResult {
            id: result,
            satus: generate_inventory.status,
        })
    }

    pub async fn add_batch(&self, data: CreateInventoryBatch) -> anyhow::Result<()> {
        let inventories = map_create_inventory_to_inventories(data)?;
        self.inventory_repository.add_batch(inventories).await?;
        Ok(())
    }
}

fn map_generate_inventory(data: GenerateInventoryDto) -> anyhow::Result<GenerateInventory> {
    let generate_inventory = GenerateInventory {
        id: Option::None,
        event_id: ObjectId::from_str(&data.event_id)?,
        number_to_create: data.number_to_create,
        created_at: chrono::Utc::now(),
        status: GENERATE_INVENTORY_STATUS_PENDING.to_string(),
    };

    Ok(generate_inventory)
}

pub fn map_create_inventory_to_inventories(
    create_inventory: CreateInventoryBatch,
) -> anyhow::Result<Vec<Inventory>> {
    let event_id = ObjectId::from_str(&create_inventory.event_id)?;
    let generate_inventory_id = create_inventory.generate_inventory_id
        .and_then(|gii| ObjectId::from_str(&gii).ok());

    let inventories = (0..create_inventory.quantity)
        .into_iter()
        .map(|_| Inventory {
            id: None,
            event_id: event_id,
            status: INVENTORY_STATUS_AVAILABLE.to_string(),
            last_status_change: Utc::now(),
            generate_inventory_id: generate_inventory_id,
        })
        .collect();
    Ok(inventories)
}
