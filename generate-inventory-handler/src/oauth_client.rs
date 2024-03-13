use std::env;

use oauth2::{
    AuthUrl, ClientId, ClientSecret, TokenUrl
};
use oauth2::basic::BasicClient;

pub fn get_client() -> anyhow::Result<BasicClient> {
    let client_id = env::var("OAuth__ClientId")
        .expect("OAuth__ClientId must be set");
    let client_secret = env::var("OAuth__ClientSecret")
        .expect("OAuth__ClientSecret must be set");

    let authorize_url = env::var("OAuth__AuthorizeUrl")
        .expect("OAuth__AuthorizeUrl must be set");

    let token_url = env::var("OAuth__TokenUrl")
        .expect("OAuth__TokenUrl must be set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(authorize_url)?,
        Some(TokenUrl::new(token_url)?),
    );

    Ok(client)
}
    

