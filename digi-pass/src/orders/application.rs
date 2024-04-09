use crate::baskets::data_transfer_objects::Basket;

pub struct  OrderService {
    
}

impl OrderService {
    pub async fn create_order(&self, Basket: Basket) -> anyhow::Result<()> {
        
        todo!("Implement OrderService::create_order")
    }
}