use mongodb::Client;

use crate::inventories::{data_transfer_objects::{ReserveInventories, ReservedInventory}, application::InventoryService};
use crate::events::{application::EventService, data_transfer_objects::EventDetails};

use super::{basket_repository::{BasketRepository, MongoDbBasketRepository}, data_models::{Basket, BasketItem, BasketedInventory}, data_transfer_objects::{AddBasketItemRequest, CreateBasketRequest, CreateBasketResult}};

#[derive(Clone)]
pub struct  BasketService {
    inventory_service: InventoryService,
    event_service: EventService,
    basket_repository: Box<dyn BasketRepository>,
}

impl BasketService {
    pub fn new(client : Client, database: String, inventory_service: InventoryService, event_service: EventService) -> Self {
        let basket_repository = Box::new(MongoDbBasketRepository::new(client.clone(), database.clone(), "Baskets".to_string()));
        Self {inventory_service, basket_repository, event_service}
    }

    pub async fn create_basket(&self, create_basket_request: CreateBasketRequest) -> anyhow::Result<CreateBasketResult> {
        
        let inventory_requests = generate_reserve_inventory_request(&create_basket_request.add_basket_item_request);
        
        let mut basket_items: Vec<BasketItem> = vec![];
        for inventory_request in inventory_requests {

            let event = self.event_service.get_event(&inventory_request.event_id).await?
                .ok_or(anyhow::anyhow!("Event not found: {:?}", inventory_request.event_id.clone()))?;
           
           
            let result =self.inventory_service.reserve_inventories(&inventory_request).await?;
            if result.reserved_inventories.len() != inventory_request.quantity as usize {
                return Err(anyhow::anyhow!("Not enough inventories: {:?}", result.reserved_inventories.len()));
            }

            let basket_item = BasketItem { 
                basketed_inventories: result.reserved_inventories.iter()
                .map(|ri| 
                    create_basketed_inventory(&event, ri))
                .collect() 
            };

            basket_items.push(basket_item);
        }
            
        //create and save basket
        let basket = Basket{  basket_items};
        let basket_id = self.basket_repository.add(basket).await?;
        
        Ok(CreateBasketResult{ basket_id : basket_id.ok_or(anyhow::anyhow!("Failed creating basket"))? })
    }
    
}

fn create_basketed_inventory(event: &EventDetails, reserved_inventory: &ReservedInventory) -> BasketedInventory {
    BasketedInventory::new(event.id.clone(), reserved_inventory.inventory_id.to_string(), reserved_inventory.reserved_until, event.price)
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



