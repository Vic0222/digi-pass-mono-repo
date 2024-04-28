use std::{str::from_utf8, sync::Arc};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use bson::oid::ObjectId;
use chrono::Utc;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use mongodb::Client;

use crate::orders::{application::OrderService, data_transfer_objects::OrderTransaction};

use super::{data_models::{Pass, PassVerification}, data_transfer_objects::JwtPass, persistence::{MongoDbPassVerificationRepository, PassVerificationRepository}};

pub struct PassService{
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    pass_verification_repository: Arc<dyn PassVerificationRepository>
}

impl PassService {
    
    
    pub fn new(private_key: String, public_key: String, client: Client, database: String) -> anyhow::Result<Self> {

        let private_key = STANDARD.decode(&private_key)
            .map_err(|_| anyhow::anyhow!("Failed decoding private key"))?;

        let private_key = from_utf8(&private_key)
            .map_err(|_| anyhow::anyhow!("Failed decoding private key"))?;
            
        let encoding_key = EncodingKey::from_rsa_pem(&private_key.as_bytes())?; // IMPROVEMENT:  Get from Key Management Service

        let public_key = STANDARD.decode(&public_key)
            .map_err(|_| anyhow::anyhow!("Failed decoding public_key key"))?;

        let public_key = from_utf8(&public_key)
            .map_err(|_| anyhow::anyhow!("Failed decoding public_key key"))?;
            
        let decoding_key = DecodingKey::from_rsa_pem(&public_key.as_bytes())?; // IMPROVEMENT:  Get from Key Management Service
        
        let pass_verification_repository = Arc::new(MongoDbPassVerificationRepository::new(client, database));
        Ok(Self {
            encoding_key,
            decoding_key,
            pass_verification_repository
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

    pub async fn verify_pass(&self, pass: JwtPass) -> anyhow::Result<bool> {
        let token = &pass.jwt;

        let decoded_token = jsonwebtoken::decode::<Pass>(token, &self.decoding_key, &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256))?;
        
        //check if already verified
        let inventory_id = ObjectId::parse_str(&decoded_token.claims.inventory_id)?;
        let pass_verification = self.pass_verification_repository.get_last_pass_verification(inventory_id).await?;
        if let Some(_) = pass_verification {
            tracing::info!("Pass already verified");
            return Ok(false);
        }
        //record pass verification
        let pass_verification = PassVerification {
            inventory_id : inventory_id,
            verification_time : Utc::now()
        };
        self.pass_verification_repository.save_pass_verification(&pass_verification).await?;
        return Ok(true);
    }
}

fn assemble_pass(order_transaction_item_inventory_id: &str, order_transaction: &OrderTransaction) -> Option<Pass> {

    let inventory_item = order_transaction.items.iter()
        .flat_map(|i| &i.inventories)
        .find(|i| i.id == order_transaction_item_inventory_id)?;
        //convert order_transaction_item_inventory to Pass{}
        let pass = Pass {
            sub: inventory_item.id.clone(),
            exp: (Utc::now().timestamp() + 3600) as usize, //TODO get from event end date
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


