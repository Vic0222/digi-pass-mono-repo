use crate::models::InventoryUpdate;

pub struct UpdateInventoriesContext<'a> {    
    pub database: String,
    pub collecion: String,
    pub inventory_updates: &'a [InventoryUpdate]
}