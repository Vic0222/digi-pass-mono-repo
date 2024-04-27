use std::{fs, str::from_utf8};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::orders::{application::OrderService, data_transfer_objects::OrderTransaction};

use super::{data_models::Pass, data_transfer_objects::JwtPass};

pub struct PassService{
    encoding_key: EncodingKey,
}

impl PassService {
    
    
    pub fn new(pass_private_key: String) -> anyhow::Result<Self> {

        let private_key = STANDARD.decode(&pass_private_key)
            .map_err(|_| anyhow::anyhow!("Failed decoding private key"))?;

        let private_key = from_utf8(&private_key)
            .map_err(|_| anyhow::anyhow!("Failed decoding private key"))?;
            
        let encoding_key = EncodingKey::from_rsa_pem(&private_key.as_bytes())?; // IMPROVEMENT:  Get from Key Management Service
        Ok(Self {
            encoding_key,
        })
    }

    
    ///converet a purhased inventory to a jwt pass
    /// TODO: get key from a Key Management Service
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
        let header = Header::new(jsonwebtoken::Algorithm::RS256);

        let token = encode(&header, &pass, &self.encoding_key)?;

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


// pub fn convert_rsa_key_to_jwks(pub_key: &str) -> anyhow::Result<Jwks> {

//     use pem::parse;
//     use ring::signature::RsaKeyPair;

//     let pem = parse(pub_key)?;
//     let rsa_keys = RsaKeyPair::from_public_key(&pem.contents)?;

//     let jwks = Jwks {
//         keys: vec![Jwk {
//             kty: "RSA".to_string(),
//             alg: "RS256".to_string(),
//             e: base64::encode(&rsa_keys.public_key().e().to_vec()),
//             n: base64::encode(&rsa_keys.public_key().n().to_vec()),
//             kid: None,
//             kty: None,
//             use_: None,
//             key_ops: None,
//             alg: None,
//         }],
//     };

//     Ok(jwks)
// }

// pub struct Jwks {
//     pub keys: Vec<Jwk>,
// }

// pub struct Jwk {
//     pub kty: String,
//     pub alg: String,
//     pub e: String,
//     pub n: String,
//     pub kid: Option<String>,
//     pub kty: Option<String>,
//     pub use_: Option<String>,
//     pub key_ops: Option<Vec<String>>,
//     pub alg: Option<String>,
// }


