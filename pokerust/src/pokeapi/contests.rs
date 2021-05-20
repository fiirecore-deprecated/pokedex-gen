use serde::{Deserialize, Serialize};

use super::berries::*;
use super::moves::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id, impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#contests-section>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ContestType {
    pub id: i16,
    pub name: String,
    pub berry_flavor: NamedAPIResource<BerryFlavor>,
    pub names: Vec<ContestName>,
}

/// <https://pokeapi.co/docs/v2.html#contestname>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ContestName {
    pub name: String,
    pub color: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#contest-effects>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ContestEffect {
    pub id: i16,
    pub appeal: u8,
    pub jam: u8,
    pub effect_entries: Vec<Effect>,
    pub flavor_text_entries: Vec<FlavorText>,
}

/// <https://pokeapi.co/docs/v2.html#super-contest-effects>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SuperContestEffect {
    pub id: i16,
    pub appeal: u8,
    pub flavor_text_entries: Vec<FlavorText>,
    pub moves: Vec<NamedAPIResource<Move>>,
}

set_endpoint!(ContestEffect, APIResourceList, "contest-effect");
set_endpoint!(SuperContestEffect, APIResourceList, "super-contest-effect");
set_endpoint!(ContestType, NamedAPIResourceList, "contest-type");

impl_id!(ContestEffect);
impl_id!(SuperContestEffect);
impl_id_and_named!(ContestType);
