use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use super::get_api_loc_from_url;
use super::utility::*;

use crate::cache::get_resource;

/// <https://pokeapi.co/docs/v2.html#un-named>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct APIResourceList<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<APIResource<T>>,
}

/// <https://pokeapi.co/docs/v2.html#named>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NamedAPIResourceList<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<NamedAPIResource<T>>,
}

/// Trait for lists of `(Named)APIResources`
#[async_trait]
pub trait List
where
    Self: Sized,
{
    /// Get the number of items in this list
    fn count(&self) -> &u64;

    /// Get the next list
    async fn next_list(&self, client: &reqwest::Client) -> Result<Option<Self>, reqwest::Error>;

    /// Get the previous list
    async fn previous_list(&self, client: &reqwest::Client) -> Result<Option<Self>, reqwest::Error>;
}

// impl<T> List for NamedAPIResourceList<T>
// where
//     T: DeserializeOwned
// {
//     fn count(&self) -> &u64 {
//         &self.count
//     }
//
//     fn next_list(&self) -> Result<Option<Self>, reqwest::Error> {
//         if let Some(loc) = &self.next {
//             let list = get_resource(get_api_loc_from_url(&loc))?.json::<Self>()?;
//             Ok(Some(list))
//         } else {
//             Ok(None)
//         }
//     }
//
//     fn previous_list(&self) -> Result<Option<Self>, reqwest::Error> {
//         if let Some(loc) = &self.next {
//             let list = get_resource(get_api_loc_from_url(&loc))?.json::<Self>()?;
//             Ok(Some(list))
//         } else {
//             Ok(None)
//         }
//     }
// }

macro_rules! impl_list {
    { $A:tt } => {
#[async_trait]
impl<T> List for $A<T>
where
    T: DeserializeOwned,
{
    fn count(&self) -> &u64 {
        &self.count
    }

    async fn next_list(&self, client: &reqwest::Client) -> Result<Option<Self>, reqwest::Error> {
        if let Some(loc) = &self.next {
            let list = get_resource(client, get_api_loc_from_url(&loc)).await?.json::<Self>().await?;
            Ok(Some(list))
        } else {
            Ok(None)
        }
    }

    async fn previous_list(&self, client: &reqwest::Client) -> Result<Option<Self>, reqwest::Error> {
        if let Some(loc) = &self.next {
            let list = get_resource(client, get_api_loc_from_url(&loc)).await?.json::<Self>().await?;
            Ok(Some(list))
        } else {
            Ok(None)
        }
    }
}
}
}

impl_list! {APIResourceList}
impl_list! {NamedAPIResourceList}
