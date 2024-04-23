use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::orders::{application::OrderService, data_transfer_objects::OrderTransaction};

use super::{data_models::Pass, data_transfer_objects::JwtPass};

pub struct PassService{

}

impl PassService {
    
    pub fn new() -> Self {
        PassService{}
    }

    pub async fn get_pass(&self, order_service: &OrderService, order_transaction_item_inventory_id: String) ->anyhow::Result<Option<JwtPass>> {

        //get order
        let id_parts: Vec<&str> = order_transaction_item_inventory_id.split('-').collect();
        let order_transaction_id = if let Some(value) = id_parts.get(0) {
            value
        }else{
            return Ok(None);
        };


        let order_transaction = order_service.get_order_transactions(order_transaction_id.to_string())
            .await?;

        let order_transaction = if let Some(order_transaction) = order_transaction {
            order_transaction
        }else{
            return Ok(None);
        };
        
        

        let pass = if let Some(pass) = assemble_pass(&order_transaction_item_inventory_id, &order_transaction){
            pass
        }else{
            return Ok(None);
        };
        //Intentionally left unused so I it will warn;
        let header = Header::default();//TODO convert to RS256
        let from_secret = EncodingKey::from_secret("secret".as_ref()); // TODO get from storage. Maybe a database.
        let token = encode(&header, &pass, &from_secret)?;

        Ok(Some(JwtPass::new(token)))
    }
}

fn assemble_pass(order_transaction_item_inventory_id: &str, order_transaction: &OrderTransaction) -> Option<Pass> {

    let inventory_item = order_transaction.items.iter()
        .flat_map(|i| &i.inventories)
        .find(|i| i.id == order_transaction_item_inventory_id)?;
        //convert order_transaction_item_inventory to Pass{}
        let pass = Pass {
            sub: inventory_item.id.clone(),
            exp: 0, //TODO get from event end date
            iat: Utc::now().timestamp() as usize,
            nbf: 0, //TODO get from event start date,
            inventory_id: inventory_item.inventory_id.clone(),
            event_id: inventory_item.event_id.clone(),
            event_name: inventory_item.name.clone(),
        };

        Some(pass)
}