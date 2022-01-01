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

use serde::{de::DeserializeOwned, Serialize};

mod client;
mod pokeapi;
mod util;

pub use client::*;
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

pub trait Endpoint {
    type ResourceListKind: DeserializeOwned + Serialize;

    const ENDPOINT: &'static str;
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