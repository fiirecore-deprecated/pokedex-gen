use serde::{Serialize, de::DeserializeOwned};

use crate::Endpoint;

pub struct Client {
    pub endpoint: String,
    pub(crate) client: reqwest::Client,
}

impl Client {

    pub(crate) async fn get_api_loc<T: DeserializeOwned>(&self, loc: &str) -> Result<T, reqwest::Error> {
        self.client.get(format!("{}{}", self.endpoint, crate::pokeapi::get_api_loc_from_url(loc))).send().await?.json::<T>().await
    }

    /// Retrieve the API object of this type with a name/id.
    pub async fn get<E: Endpoint + DeserializeOwned + Serialize, I: std::fmt::Display>(&self, id: I) -> Result<E, reqwest::Error> {
        self.client.get(format!("{}{}/{}/", self.endpoint, E::ENDPOINT, id)).send().await?.json::<E>().await
    }

    /// Get a list of these API objects with the given offset and limit.
    pub async fn list<E: Endpoint + DeserializeOwned + Serialize>(&self, offset: usize, limit: usize) -> Result<E::ResourceListKind, reqwest::Error> {
        self.client.get(format!("{}{}/?offset={}&limit={}", self.endpoint, E::ENDPOINT, offset, limit)).send().await?.json::<E::ResourceListKind>().await
    }

    /// Get the complete list of these API objects.
    pub async fn full_list<E: Endpoint + DeserializeOwned + Serialize>(&self) -> Result<E::ResourceListKind, reqwest::Error> {
        self.client.get(format!("{}{}/?offset=0&limit=9999", self.endpoint, E::ENDPOINT)).send().await?.json::<E::ResourceListKind>().await
    }

}

impl Default for Client {
    fn default() -> Self {
        Self {
            endpoint: match std::env::var("POKERUST_ENDPOINT") {
                Ok(val) => val,
                Err(std::env::VarError::NotPresent) => String::from("https://pokeapi.co/api/v2/"),
                Err(err) => panic!("Could not read endpoint from POKERUST_ENDPOINT with error {}", err),
            },
            client: reqwest::Client::new(),
        }
    }
}