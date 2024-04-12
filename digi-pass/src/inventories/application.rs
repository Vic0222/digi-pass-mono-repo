use std::str::FromStr;
use std::sync::Arc;

use anyhow::Ok;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::Client;
use crate::events::application::EventService;

use super::constants::{GENERATE_INVENTORY_STATUS_PENDING, INVENTORY_STATUS_AVAILABLE, INVENTORY_STATUS_RESERVED};
use super::data_models::{GenerateInventory, Inventory};
use super::data_transfer_objects::{
    CreateInventoryBatch, GenerateInventory as GenerateInventoryDto,
    GenerateInventoryResult, ReserveInventories, ReserveInventoriesResult, ReservedInventory,
};
use super::inventory_repository::{InventoryRepository, MongoDbInventoryRepository};

#[derive(Clone)]
pub struct InventoryService {
    pub inventory_repository: Arc<dyn InventoryRepository>,
    pub event_service: EventService,
}

impl InventoryService {

    pub fn new(client: Client, database: String, event_service: EventService) -> Self {
        let inventory_repository = Arc::new(MongoDbInventoryRepository::new(client, database, "Inventories".to_string()));
        InventoryService {
            inventory_repository,
            event_service
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

    pub async fn reserve_inventories(&self, reserve_inventories: &ReserveInventories) -> anyhow::Result<ReserveInventoriesResult> {
        let _event = self.event_service.get_event(&reserve_inventories.event_id).await?.ok_or(anyhow::anyhow!("Event not found"))?;
        
        let now = Utc::now();
        let reserved_until = now + chrono::Duration::minutes(90);
        let mut inventories = self.inventory_repository.get_unreserved_inventories(reserve_inventories.event_id.clone(), reserve_inventories.quantity, now, INVENTORY_STATUS_AVAILABLE).await?;
        if inventories.len()  != reserve_inventories.quantity as usize {
            return Err(anyhow::anyhow!("Not enough inventories: {:?}", inventories.len()));
        }
        for inventory in inventories.iter_mut() {
            inventory.reserved_until = reserved_until;
            inventory.status = INVENTORY_STATUS_RESERVED.to_string();
        }
        self.inventory_repository.batch_update_reservations(&inventories).await?;
        Ok(ReserveInventoriesResult {
            reserved_inventories : inventories.iter()
                .filter_map(|inventory|  
                    inventory.id.map(|id| ReservedInventory::new(id.to_hex(), inventory.reserved_until)) )
                .collect()
        })
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
        .map(|_| Inventory {
            id: None,
            event_id,
            status: INVENTORY_STATUS_AVAILABLE.to_string(),
            reserved_until: Utc::now(),
            generate_inventory_id,
            concurrency_stamp : ObjectId::new().to_hex()
        })
        .collect();
    Ok(inventories)
}
