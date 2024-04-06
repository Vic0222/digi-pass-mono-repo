use std::sync::Arc;

use chrono::Utc;
use mongodb::Client;

use crate::events::{application::EventService, data_transfer_objects::EventDetails};
use crate::{
    inventories::{
        application::InventoryService,
        data_transfer_objects::{ReserveInventories, ReservedInventory},
    },
    payments::{application::PaymentService, data_transfer_objects::PaymentView},
};

use super::{
    basket_repository::{BasketRepository, MongoDbBasketRepository},
    data_models::{self, Basket, BasketItem, BasketedInventory},
    data_transfer_objects::{self, AddBasketItemRequest, CreateBasketRequest, CreateBasketResult},
};

#[derive(Clone)]
pub struct BasketService {
    inventory_service: InventoryService,
    event_service: EventService,
    basket_repository: Box<dyn BasketRepository>,
}

impl BasketService {
    pub fn new(
        client: Client,
        database: String,
        inventory_service: InventoryService,
        event_service: EventService,
    ) -> Self {
        let basket_repository = Box::new(MongoDbBasketRepository::new(
            client.clone(),
            database.clone(),
            "Baskets".to_string(),
        ));
        Self {
            inventory_service,
            basket_repository,
            event_service,
        }
    }

    pub async fn create_basket(
        &self,
        create_basket_request: CreateBasketRequest,
    ) -> anyhow::Result<CreateBasketResult> {
        let inventory_requests =
            generate_reserve_inventory_request(&create_basket_request.add_basket_item_request);

        let mut basket_items: Vec<BasketItem> = vec![];
        for inventory_request in inventory_requests {
            let event = self
                .event_service
                .get_event(&inventory_request.event_id)
                .await?
                .ok_or(anyhow::anyhow!(
                    "Event not found: {:?}",
                    inventory_request.event_id.clone()
                ))?;

            let result = self
                .inventory_service
                .reserve_inventories(&inventory_request)
                .await?;
            if result.reserved_inventories.len() != inventory_request.quantity as usize {
                return Err(anyhow::anyhow!(
                    "Not enough inventories: {:?}",
                    result.reserved_inventories.len()
                ));
            }

            let basket_item = BasketItem {
                basketed_inventories: result
                    .reserved_inventories
                    .iter()
                    .map(|ri| create_basketed_inventory(&event, ri))
                    .collect(),
            };

            basket_items.push(basket_item);
        }

        //create and save basket
        let valid_until = Utc::now() + chrono::Duration::minutes(30);
        let basket = Basket::new(valid_until, basket_items);
        let basket_id = self.basket_repository.add(basket).await?;

        Ok(CreateBasketResult {
            basket_id: basket_id.ok_or(anyhow::anyhow!("Failed creating basket"))?,
        })
    }

    pub async fn get_valid_basket(
        &self,
        basket_id: &str,
    ) -> anyhow::Result<Option<data_transfer_objects::Basket>> {
        let basket = self.basket_repository.get(basket_id).await?;

        match basket {
            None => Ok(None),
            Some(basket) => {
                if !is_all_inventory_reserved(&basket) {
                    return Ok(None);
                }
                if is_basket_expired(&basket) {
                    return Ok(None);
                }
                Ok(Some(map_dto_basket_to_data_basket(&basket)?))
            }
        }
    }

    pub async fn purchase_basket(
        &self,
        payment_service: &PaymentService,
        basket_id: &str,
    ) -> anyhow::Result<()> {
        let basket = self.basket_repository.get(basket_id).await?;

        let basket = basket.ok_or(anyhow::anyhow!("Basket not found"))?;
        if !is_all_inventory_reserved(&basket) {
            return Err(anyhow::anyhow!("Not all inventory reserved"));
        }
        if is_basket_expired(&basket) {
            return Err(anyhow::anyhow!("Basket expired"));
        }

        let basket_payments = payment_service.get_basket_payments(basket_id).await?;

        let paid_payments: i32 = basket_payments
            .iter()
            .filter_map(|p| {
                if p.status == "paid" {
                    return Some(p.amount);
                } else {
                    return None;
                }
            })
            .sum();
        if paid_payments < compute_basket_total_price(&basket) {
            return Err(anyhow::anyhow!("Basket not fully paid."));
        }
        todo!("Convert basket to order");
    }
}

fn is_basket_expired(basket: &Basket) -> bool {
    return basket.valid_until < Utc::now();
}

fn compute_basket_total_price(basket: &Basket) -> i32 {
    let total_price = basket
        .basket_items
        .iter()
        .map(|bi| {
            bi.basketed_inventories
                .iter()
                .map(|basketed_inventory| basketed_inventory.price)
                .sum::<i32>()
        })
        .sum();

    return total_price;
}

fn map_dto_basket_to_data_basket(
    data_basket: &data_models::Basket,
) -> anyhow::Result<data_transfer_objects::Basket> {
    let mut total_price = 0;
    let mut basket_items = vec![];

    for basket_item in data_basket.basket_items.iter() {
        let mut basket_item_total_price = 0;
        for basketed_inventory in basket_item.basketed_inventories.iter() {
            total_price += basketed_inventory.price;
            basket_item_total_price += basketed_inventory.price;
        }

        let dto_basket_item = data_transfer_objects::BasketItem {
            basketed_inventories: basket_item
                .basketed_inventories
                .iter()
                .map(
                    |basketed_inventory| data_transfer_objects::BasketedInventory {
                        event_id: basketed_inventory.event_id.clone(),
                        name: basketed_inventory.name.clone(),
                        inventory_id: basketed_inventory.inventory_id.clone(),
                        reserved_until: basketed_inventory.reserved_until,
                        price: basketed_inventory.price,
                    },
                )
                .collect(),
            total_price: basket_item_total_price,
        };
        basket_items.push(dto_basket_item);
    }

    let dto_basket = data_transfer_objects::Basket {
        id: data_basket
            .id
            .map(|id| id.to_hex())
            .ok_or(anyhow::anyhow!("No basket id!"))?,
        valid_until: data_basket.valid_until,
        basket_items,
        total_price,
    };
    Ok(dto_basket)
}

fn is_all_inventory_reserved(basket: &Basket) -> bool {
    for basket_item in basket.basket_items.iter() {
        for basketed_inventory in basket_item.basketed_inventories.iter() {
            if basketed_inventory.reserved_until < Utc::now() {
                return false;
            }
        }
    }
    true
}

fn create_basketed_inventory(
    event: &EventDetails,
    reserved_inventory: &ReservedInventory,
) -> BasketedInventory {
    BasketedInventory::new(
        event.id.clone(),
        event.name.clone(),
        reserved_inventory.inventory_id.to_string(),
        reserved_inventory.reserved_until,
        event.price,
    )
}

fn generate_reserve_inventory_request(
    add_basket_item_requests: &[AddBasketItemRequest],
) -> Vec<ReserveInventories> {
    add_basket_item_requests
        .iter()
        .map(|add_basket_item_request| ReserveInventories {
            event_id: add_basket_item_request.event_id.clone(),
            quantity: add_basket_item_request.quantity,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_reserve_inventory_request() {
        let add_basket_item_requests = vec![
            AddBasketItemRequest {
                event_id: 1.to_string(),
                quantity: 2,
            },
            AddBasketItemRequest {
                event_id: 2.to_string(),
                quantity: 3,
            },
        ];
        let expected = vec![
            ReserveInventories {
                event_id: 1.to_string(),
                quantity: 2,
            },
            ReserveInventories {
                event_id: 2.to_string(),
                quantity: 3,
            },
        ];

        let actuals = generate_reserve_inventory_request(&add_basket_item_requests);

        assert_eq!(&actuals[0].event_id, &expected[0].event_id);
        assert_eq!(&actuals[0].quantity, &expected[0].quantity);

        assert_eq!(&actuals[1].event_id, &expected[1].event_id);
        assert_eq!(&actuals[1].quantity, &expected[1].quantity);
    }
}
