use serde::{Deserialize, Serialize};

use super::evolution::*;
use super::games::*;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#item>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Item {
    pub id: i16,
    pub name: String,
    pub cost: u32,
    pub fling_power: Option<u8>,
    pub fling_effect: Option<NamedAPIResource<ItemFlingEffect>>,
    pub attributes: Vec<NamedAPIResource<ItemAttribute>>,
    pub category: NamedAPIResource<ItemCategory>, // incorrectly documented as ItemCategory
    pub effect_entries: Vec<VerboseEffect>,
    pub flavor_text_entries: Vec<VersionGroupFlavorText>,
    pub game_indices: Vec<GenerationGameIndex>,
    pub names: Vec<Name>,
    pub sprites: ItemSprites,
    pub held_by_pokemon: Vec<ItemHolderPokemon>,
    pub baby_trigger_for: Option<APIResource<EvolutionChain>>,
    pub machines: Vec<MachineVersionDetail>,
}

/// <https://pokeapi.co/docs/v2.html#itemsprites>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemSprites {
    pub default: String,
}

/// <https://pokeapi.co/docs/v2.html#itemholderpokemon>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemHolderPokemon {
    pub pokemon: NamedAPIResource<Pokemon>, // incorrectly documented as string
    pub version_details: Vec<ItemHolderPokemonVersionDetail>,
}

/// <https://pokeapi.co/docs/v2.html#itemholderpokemonversiondetail>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemHolderPokemonVersionDetail {
    pub rarity: u8, // incorrectly documented as string
    pub version: NamedAPIResource<Version>,
}

/// <https://pokeapi.co/docs/v2.html#item-attributes>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemAttribute {
    pub id: i16,
    pub name: String,
    pub items: Vec<NamedAPIResource<Item>>,
    pub names: Vec<Name>,
    pub descriptions: Vec<Description>,
}

/// <https://pokeapi.co/docs/v2.html#item-categories>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemCategory {
    pub id: i16,
    pub name: String,
    pub items: Vec<NamedAPIResource<Item>>,
    pub names: Vec<Name>,
    pub pocket: NamedAPIResource<ItemPocket>,
}

/// <https://pokeapi.co/docs/v2.html#item-fling-effects>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemFlingEffect {
    pub id: i16,
    pub name: String,
    pub effect_entries: Vec<Effect>,
    pub items: Vec<NamedAPIResource<Item>>, // incorrectly documented as NamedAPIResource
}

/// <https://pokeapi.co/docs/v2.html#item-pockets>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemPocket {
    pub id: i16,
    pub name: String,
    pub categories: Vec<NamedAPIResource<ItemCategory>>,
    pub names: Vec<Name>,
}

set_endpoint!(Item, NamedAPIResourceList, "item");
set_endpoint!(ItemAttribute, NamedAPIResourceList, "item-attribute");
set_endpoint!(ItemCategory, NamedAPIResourceList, "item-category");
set_endpoint!(ItemFlingEffect, NamedAPIResourceList, "item-fling-effect");
set_endpoint!(ItemPocket, NamedAPIResourceList, "item-pocket");

impl_id_and_named!(Item);
impl_id_and_named!(ItemAttribute);
impl_id_and_named!(ItemCategory);
impl_id_and_named!(ItemFlingEffect);
impl_id_and_named!(ItemPocket);
