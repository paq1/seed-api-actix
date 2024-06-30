pub mod authenticated;
pub mod jwt_claims;

use std::fmt::Debug;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::core::shared::token::TokenService;

pub struct JwtTokenService {
    secret: String
}

impl JwtTokenService {
    pub fn new(secret: String) -> Self {
        Self {
            secret
        }
    }
}

impl TokenService for JwtTokenService {

    fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> Result<CLAIMS, String> {
        decode::<CLAIMS>(token, &DecodingKey::from_secret(self.secret.as_bytes()), &Validation::default())
            .map(|token_data| token_data.claims)
            .map_err(|err| err.to_string())
    }
}
