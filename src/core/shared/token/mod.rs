use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait TokenService {
    fn decode<CLAIMS: Debug + Serialize + DeserializeOwned>(&self, token: &str) -> Result<CLAIMS, String>;
}