use serde::{Deserialize, Serialize};

use super::contests::*;
use super::items::*;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#berries>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Berry {
    pub id: i16,
    pub name: String,
    pub growth_time: u8,
    pub max_harvest: u8,
    pub natural_gift_power: u8,
    pub size: u16,
    pub smoothness: u8,
    pub soil_dryness: u8,
    pub firmness: NamedAPIResource<BerryFirmness>,
    pub flavors: Vec<BerryFlavorMap>,
    pub item: NamedAPIResource<Item>,
    pub natural_gift_type: NamedAPIResource<Type>,
}

/// <https://pokeapi.co/docs/v2.html#berryflavormap>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct BerryFlavorMap {
    pub potency: u8,
    pub flavor: NamedAPIResource<BerryFlavor>,
}

/// <https://pokeapi.co/docs/v2.html#berry-firmnesses>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct BerryFirmness {
    pub id: i16,
    pub name: String,
    pub berries: Vec<NamedAPIResource<Berry>>,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#berry-flavors>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct BerryFlavor {
    pub id: i16,
    pub name: String,
    pub berries: Vec<FlavorBerryMap>,
    pub contest_type: NamedAPIResource<ContestType>,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#flavorberrymap>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct FlavorBerryMap {
    pub potency: u8,
    pub berry: NamedAPIResource<Berry>,
}

set_endpoint!(Berry, NamedAPIResourceList, "berry");
set_endpoint!(BerryFirmness, NamedAPIResourceList, "berry-firmness");
set_endpoint!(BerryFlavor, NamedAPIResourceList, "berry-flavor");

impl_id_and_named!(Berry);
impl_id_and_named!(BerryFirmness);
impl_id_and_named!(BerryFlavor);
