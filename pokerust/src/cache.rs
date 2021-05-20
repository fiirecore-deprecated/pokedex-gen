// use cached::cached_key_result;
// use cached::UnboundCache;
use lazy_static::lazy_static;
use reqwest::Response;
use std::env;

lazy_static! {
    /// Pokeapi endpoint. Can be set with the `POKERUST_ENDPOINT` environment
    /// variable. Defaults to <https://pokeapi.co/api/v2/>.
    pub static ref ENDPOINT: String = match env::var("POKERUST_ENDPOINT") {
        Ok(val) => val,
        Err(env::VarError::NotPresent) => String::from("https://pokeapi.co/api/v2/"),
        Err(_) => panic!("Error reading endpoint from POKERUST_ENDPOINT"),
    };
}

// cached_key_result! {
//    POKEAPI_CACHE: UnboundCache<String, Response> = UnboundCache::new();
//    Key = { path.to_owned() };
   
// }

pub async fn get_resource(client: &reqwest::Client, path: &str) -> Result<Response, reqwest::Error> {
    client.get(&format!("{}{}", *ENDPOINT, path)).send().await
}