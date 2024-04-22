use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JwtPass {
    pub jwt: String
}

impl JwtPass {
    pub fn new(jwt: String) -> Self {
        Self {
            jwt
        }
    }
    
}