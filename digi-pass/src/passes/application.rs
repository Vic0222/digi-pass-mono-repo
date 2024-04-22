use jsonwebtoken::{encode, EncodingKey, Header};

use super::{data_models::Pass, data_transfer_objects::JwtPass};

pub struct PassService{

}

impl PassService {
    
    pub fn new() -> Self {
        PassService{}
    }

    pub async fn get_pass(&self, order_transaction_item_inventory_id: String) ->anyhow::Result<JwtPass> {

        let pass = Pass {
            sub: "test".to_string(),
            exp: 0,
            iat: 0,
            iss: "test".to_string(),
            nbf: 0,
            inventory_id: order_transaction_item_inventory_id,
            event_id: "test".to_string(),
            event_name: "test".to_string(),
            event_description: "test".to_string()
        };
        let token = encode(&Header::default(), &pass, &EncodingKey::from_secret("secret".as_ref()))?;

        Ok(JwtPass::new(token))
    }
}