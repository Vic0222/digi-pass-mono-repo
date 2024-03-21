

use chrono::Utc;

use crate::inventories::{data_transfer_objects::ReserveInventories, inventory_manager::InventoryManager};

use super::{basket_repository::BasketRepository, data_models::{Basket, BasketItem, BasketedInventory}, data_transfer_objects::{AddBasketItemRequest, CreateBasketRequest, CreateBasketResult}};

pub struct  BasketManager {
    inventory_manager: InventoryManager,
    basket_repository: Box<dyn BasketRepository>,
}

impl BasketManager {
    pub fn new(inventory_manager: InventoryManager, basket_repository: Box<dyn BasketRepository>) -> Self {
        Self {inventory_manager, basket_repository}
    }

    pub async fn create_basket(&self, create_basket_request: CreateBasketRequest) -> anyhow::Result<CreateBasketResult> {
        
        let inventory_requests = generate_reserve_inventory_request(&create_basket_request.add_basket_item_request);
        
        let mut basket_items: Vec<BasketItem> = vec![];
        for inventory_request in inventory_requests {
            let result =self.inventory_manager.reserve_inventories(&inventory_request).await?;
            if result.reserved_inventories.len() != inventory_request.quantity as usize {
                return Err(anyhow::anyhow!("Not enough inventories: {:?}", result.reserved_inventories.len()));
            }

            let basket_item = BasketItem { 
                basketed_inventories: result.reserved_inventories.iter()
                    .map(|ri| 
                        BasketedInventory::new(inventory_request.event_id.clone(), ri.inventory_id.to_string(), ri.reserved_until))
                    .collect() };

            basket_items.push(basket_item);
        }
            
        //create and save basket
        let basket = Basket{  basket_items};
        let basket_id = self.basket_repository.add(basket).await?;
        
        Ok(CreateBasketResult{ basket_id : basket_id.ok_or(anyhow::anyhow!("Failed creating basket"))? })
    }
    
}

fn generate_reserve_inventory_request(add_basket_item_requests: &Vec<AddBasketItemRequest>) -> Vec<ReserveInventories> {

    add_basket_item_requests.iter().map(|add_basket_item_request| {
        ReserveInventories{ event_id: add_basket_item_request.event_id.clone(), quantity: add_basket_item_request.quantity  }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_generate_reserve_inventory_request() {
        let add_basket_item_requests = vec![
            AddBasketItemRequest { event_id: 1.to_string(), quantity: 2 },
            AddBasketItemRequest { event_id: 2.to_string(), quantity: 3 },
        ];
        let expected = vec![
            ReserveInventories { event_id: 1.to_string(), quantity: 2},
            ReserveInventories { event_id: 2.to_string(), quantity: 3},
        ];

        let actuals = generate_reserve_inventory_request(&add_basket_item_requests);

        assert_eq!(&actuals[0].event_id, &expected[0].event_id);
        assert_eq!(&actuals[0].quantity, &expected[0].quantity);

        assert_eq!(&actuals[1].event_id, &expected[1].event_id);
        assert_eq!(&actuals[1].quantity, &expected[1].quantity);
    }

    
}



