//! # Pokérust
//!
//! Rust wrapper for the pokeapi <https://pokeapi.co/>
//!
//! ## Basic Usage
//!
//! Get an object from an API by id
//! ```no_run
//! use pokerust::{Berry, FromId};
//!
//! fn main() {
//!     let berry = Berry::from_id(1).unwrap();
//! }
//! ```
//! or by name
//! ```no_run
//! use pokerust::{Berry, FromName};
//!
//! fn main() {
//!     let berry = Berry::from_name("cheri").unwrap();
//! }
//! ```
//! API responses are automatically cached.
//!
//! You can also fetch the resource lists:
//! ```no_run
//! # use std::error::Error;
//! # use pokerust::{Item, Endpoint, List};
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let items = Item::list(5, 20)?;  // ?offset=5&limit=20
//!
//! // get the lists referenced in the next and previous fields
//! items.previous_list()?;
//! items.next_list()?;
//!
//! // you can also just get the full list
//! let all_items = Item::full_list()?;
//! # Ok(())
//! # }
//! ```
//!
//! To get resources pointed to by `(Named)APIResource`, use `get()`:
//! ```no_run
//! # use std::error::Error;
//! # use pokerust::{Berry, FromName};
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let berry = Berry::from_name("cheri")?;
//! let berry_item = berry.item.get()?; // berry_item is an Item
//! # Ok(())
//! # }
//! ```
//! This can be chained:
//! ```no_run
//! # use std::error::Error;
//! # use pokerust::{PokemonSpecies, FromName};
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let marill = PokemonSpecies::from_name("marill")?;
//! let sea_incense = marill.evolution_chain.get()?.baby_trigger_item.unwrap().get()?;
//! # Ok(())
//! # }
//! ```
//!
//! The location of the pokeapi used can be changed by setting the
//! POKERUST_ENDPOINT environment variable. Defaults to the public instance at
//! <https://pokeapi.co/api/v2/>. Please consult the pokeapi documentation and read
//! the fair use policy before using the public API instance.

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use serde::de::DeserializeOwned;
use async_trait::async_trait;
use crate::cache::get_resource;

mod cache;
mod pokeapi;
mod util;

pub use cache::ENDPOINT;
pub use pokeapi::berries::*;
pub use pokeapi::contests::*;
pub use pokeapi::encounters::*;
pub use pokeapi::evolution::*;
pub use pokeapi::games::*;
pub use pokeapi::items::*;
pub use pokeapi::locations::*;
pub use pokeapi::machines::*;
pub use pokeapi::moves::*;
pub use pokeapi::pokemon::*;
pub use pokeapi::resource_lists::*;
pub use pokeapi::utility::*;

/// Trait for API objects with an associated endpoint.
#[async_trait]
pub trait Endpoint {
    type ResourceListKind;

    const ENDPOINT: &'static str;

    /// Get a list of these API objects with the given offset and limit.
    async fn list(client: &reqwest::Client, offset: usize, limit: usize) -> Result<Self::ResourceListKind, reqwest::Error>;

    /// Get the complete list of these API objects.
    async fn full_list(client: &reqwest::Client) -> Result<Self::ResourceListKind, reqwest::Error>;
}

/// Trait for API objects with a name.
pub trait Named {
    /// Get the name of this object.
    fn name(&self) -> &String;
}

/// Trait for API objects with an ID.
pub trait Id {
    /// Get the ID of this object.
    fn id(&self) -> i16;
}

/// API resources that can be retrived from an ID.
#[async_trait]
pub trait FromId: Id + Endpoint
where
    Self: Sized,
{
    /// Retrieve the API object of this type with this ID.
    async fn from_id(client: &reqwest::Client, id: i16) -> Result<Self, reqwest::Error>;
}

#[async_trait]
impl<T: Endpoint + Id + DeserializeOwned> FromId for T {
    async fn from_id(client: &reqwest::Client, id: i16) -> Result<Self, reqwest::Error> {
        get_resource(client, &format!("{}/{}/", T::ENDPOINT, id)).await?.json::<Self>().await
    }
}

/// API resources that can be retrived from a name.
#[async_trait]
pub trait FromName: Named + Endpoint
where
    Self: Sized,
{
    /// Retrieve the API object of this type with this name.
    async fn from_name(client: &reqwest::Client, id: &str) -> Result<Self, reqwest::Error>;
}

#[async_trait]
impl<T: Endpoint + Named + DeserializeOwned> FromName for T {
    async fn from_name(client: &reqwest::Client, id: &str) -> Result<Self, reqwest::Error> {
        get_resource(client, &format!("{}/{}/", T::ENDPOINT, id)).await?.json::<Self>().await
    }
}
