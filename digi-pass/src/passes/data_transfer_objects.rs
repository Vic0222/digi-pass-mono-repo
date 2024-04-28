use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct JwtPass {
    #[validate(length(min = 1))]
    pub jwt: String
}

impl JwtPass {
    pub fn new(jwt: String) -> Self {
        Self {
            jwt
        }
    }
    
}

#[derive(Validate, Debug, Serialize)]
pub struct  VerificationResult {
    pub valid: bool,
}

impl VerificationResult {
    pub fn new(valid: bool) -> Self {
        Self {
            valid
        }
    }
}

