use serde::{Deserialize, Serialize};

use super::locations::*;
use super::moves::*;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#generations>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Generation {
    pub id: i16,
    pub name: String,
    pub abilities: Vec<NamedAPIResource<Ability>>,
    pub names: Vec<Name>,
    pub main_region: NamedAPIResource<Region>,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
    pub types: Vec<NamedAPIResource<Type>>,
    pub version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

/// <https://pokeapi.co/docs/v2.html#pokedexes>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Pokedex {
    pub id: i16,
    pub name: String,
    pub is_main_series: bool,
    pub descriptions: Vec<Description>,
    pub names: Vec<Name>,
    pub pokemon_entries: Vec<PokemonEntry>,
    pub region: Option<NamedAPIResource<Region>>,
    pub version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonentry>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonEntry {
    pub entry_number: u16,
    pub pokemon_species: NamedAPIResource<PokemonSpecies>,
}

/// <https://pokeapi.co/docs/v2.html#version>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Version {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

/// <https://pokeapi.co/docs/v2.html#version-groups>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct VersionGroup {
    pub id: i16,
    pub name: String,
    pub order: u16,
    pub generation: NamedAPIResource<Generation>,
    pub move_learn_methods: Vec<NamedAPIResource<MoveLearnMethod>>,
    pub pokedexes: Vec<NamedAPIResource<Pokedex>>,
    pub regions: Vec<NamedAPIResource<Region>>,
    pub versions: Vec<NamedAPIResource<Version>>,
}

set_endpoint!(Generation, NamedAPIResourceList, "generation");
set_endpoint!(Pokedex, NamedAPIResourceList, "pokedex");
set_endpoint!(Version, NamedAPIResourceList, "version");
set_endpoint!(VersionGroup, NamedAPIResourceList, "version-group");

impl_id_and_named!(Generation);
impl_id_and_named!(Pokedex);
impl_id_and_named!(Version);
impl_id_and_named!(VersionGroup);
