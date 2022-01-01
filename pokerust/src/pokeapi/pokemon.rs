use serde::{Deserialize, Serialize};

use super::berries::*;
use super::evolution::*;
use super::games::*;
use super::items::*;
use super::locations::*;
use super::moves::*;
use super::resource_lists::*;
use super::utility::*;
use crate::{impl_id, impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#abilities>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Ability {
    pub id: i16,
    pub name: String,
    pub is_main_series: bool,
    pub generation: NamedAPIResource<Generation>,
    pub names: Vec<Name>,
    pub effect_entries: Vec<VerboseEffect>,
    pub effect_changes: Vec<AbilityEffectChange>,
    pub flavor_text_entries: Vec<AbilityFlavorText>,
    pub pokemon: Vec<AbilityPokemon>,
}

/// <https://pokeapi.co/docs/v2.html#abilityeffectchange>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AbilityEffectChange {
    pub effect_entries: Vec<Effect>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

/// <https://pokeapi.co/docs/v2.html#abilityflavortext>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AbilityFlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource<Language>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

/// <https://pokeapi.co/docs/v2.html#abilitypokemon>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AbilityPokemon {
    pub is_hidden: bool,
    pub slot: u8,
    pub pokemon: NamedAPIResource<Pokemon>,
}

/// <https://pokeapi.co/docs/v2.html#characteristics>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Characteristic {
    pub id: i16,
    pub gene_modulo: i8,
    pub possible_values: Vec<u8>,
    pub highest_stat: NamedAPIResource<Stat>, // not documented
    pub descriptions: Vec<Description>,       // not documented
}

/// <https://pokeapi.co/docs/v2.html#egg-groups>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct EggGroup {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

/// <https://pokeapi.co/docs/v2.html#genders>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Gender {
    pub id: i16,
    pub name: String,
    pub pokemon_species_details: Vec<PokemonSpeciesGender>,
    pub required_for_evolution: Vec<NamedAPIResource<PokemonSpecies>>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonspeciesgender>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonSpeciesGender {
    pub rate: i8,
    pub pokemon_species: NamedAPIResource<PokemonSpecies>,
}

/// <https://pokeapi.co/docs/v2.html#growth-rates>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GrowthRate {
    pub id: i16,
    pub name: String,
    pub formula: String,
    pub descriptions: Vec<Description>,
    pub levels: Vec<GrowthRateExperienceLevel>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

/// <https://pokeapi.co/docs/v2.html#growthrateexperiencelevel>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GrowthRateExperienceLevel {
    pub level: u8,
    pub experience: u32,
}

/// <https://pokeapi.co/docs/v2.html#natures>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Nature {
    pub id: i16,
    pub name: String,
    pub decreased_stat: Option<NamedAPIResource<Stat>>,
    pub increased_stat: Option<NamedAPIResource<Stat>>,
    pub hates_flavor: Option<NamedAPIResource<BerryFlavor>>,
    pub likes_flavor: Option<NamedAPIResource<BerryFlavor>>,
    pub pokeathlon_stat_changes: Vec<NatureStatChange>,
    pub move_battle_style_preferences: Vec<MoveBattleStylePreference>,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#naturestatchange>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NatureStatChange {
    pub max_change: i8,
    pub pokeathlon_stat: NamedAPIResource<PokeathlonStat>,
}

/// <https://pokeapi.co/docs/v2.html#movebattlestylepreference>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveBattleStylePreference {
    pub low_hp_preference: u8,
    pub high_hp_preference: u8,
    pub move_battle_style: NamedAPIResource<MoveBattleStyle>,
}

/// <https://pokeapi.co/docs/v2.html#pokeathlon-stats>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokeathlonStat {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub affecting_natures: NaturePokeathlonStatAffectSets,
}

/// <https://pokeapi.co/docs/v2.html#naturepokeathlonstataffectsets>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NaturePokeathlonStatAffectSets {
    pub increase: Vec<NaturePokeathlonStatAffect>,
    pub decrease: Vec<NaturePokeathlonStatAffect>,
}

/// <https://pokeapi.co/docs/v2.html#naturepokeathlonstataffect>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NaturePokeathlonStatAffect {
    pub max_change: i8,
    pub nature: NamedAPIResource<Nature>,
}

/// <https://pokeapi.co/docs/v2.html#pokemon>
//////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Pokemon {
    pub id: i16,
    pub name: String,
    pub base_experience: u16,
    pub height: u8,
    pub is_default: bool,
    pub order: u16,
    pub weight: u16,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<NamedAPIResource<PokemonForm>>,
    pub game_indices: Vec<VersionGameIndex>,
    pub held_items: Vec<PokemonHeldItem>,
    pub location_area_encounters: String, // TODO: implement a way to retrieve these
    pub moves: Vec<PokemonMove>,
    pub sprites: PokemonSprites,
    pub species: NamedAPIResource<PokemonSpecies>,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonType>,
}

impl Pokemon {
    /// Fetch list of `LocationAreaEncounters` from the API for this Pokemon.
    pub fn get_encounters(&self, client: &crate::Client) -> Result<Vec<LocationAreaEncounter>, attohttpc::Error> {
        client.get_api_loc(&self.location_area_encounters)
    }
}

/// <https://pokeapi.co/docs/v2.html#pokemonability>
//////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: u8,
    pub ability: NamedAPIResource<Ability>,
}

/// <https://pokeapi.co/docs/v2.html#pokemontype>
//////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonType {
    pub slot: u8,
    #[serde(rename = "type")]
    pub type_: NamedAPIResource<Type>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonhelditem>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonHeldItem {
    pub item: NamedAPIResource<Item>,
    pub version_details: Vec<PokemonHeldItemVersion>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonhelditemversion>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonHeldItemVersion {
    pub version: NamedAPIResource<Version>,
    pub rarity: u8,
}

/// <https://pokeapi.co/docs/v2.html#pokemonmove>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub move_: NamedAPIResource<Move>,
    pub version_group_details: Vec<PokemonMoveVersion>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonmoveversion>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonMoveVersion {
    pub move_learn_method: NamedAPIResource<MoveLearnMethod>,
    pub version_group: NamedAPIResource<VersionGroup>,
    pub level_learned_at: u8,
}

/// <https://pokeapi.co/docs/v2.html#pokemonstat>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonStat {
    pub stat: NamedAPIResource<Stat>,
    pub effort: u8,
    pub base_stat: u8,
}

/// <https://pokeapi.co/docs/v2.html#pokemonsprites>
//////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
}

/// <https://pokeapi.co/docs/v2.html#locationareaencounter>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct LocationAreaEncounter {
    pub location_area: NamedAPIResource<LocationArea>,
    pub version_details: Vec<VersionEncounterDetail>,
}

/// <https://pokeapi.co/docs/v2.html#pokemon-colors>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonColor {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

/// <https://pokeapi.co/docs/v2.html#pokemon-forms>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonForm {
    pub id: i16,
    pub name: String,
    pub order: u16,
    pub form_order: u16,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub form_name: String,
    pub pokemon: NamedAPIResource<Pokemon>,
    pub sprites: PokemonFormSprites,
    pub version_group: NamedAPIResource<VersionGroup>,
    pub names: Vec<Name>,
    pub form_names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonformsprites>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonFormSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
}

/// <https://pokeapi.co/docs/v2.html#pokemon-habitats>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonHabitat {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

/// <https://pokeapi.co/docs/v2.html#pokemon-shapes>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonShape {
    pub id: i16,
    pub name: String,
    pub awesome_names: Vec<AwesomeName>,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>, // incorrectly documented as list PokemonSpecies
}

/// <https://pokeapi.co/docs/v2.html#awesomename>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct AwesomeName {
    pub awesome_name: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#pokemon-species>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonSpecies {
    pub id: i16,
    pub name: String,
    pub order: u16,
    pub gender_rate: i8,
    pub capture_rate: u8,
    pub base_happiness: u8,
    pub is_baby: bool,
    pub hatch_counter: u8,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub growth_rate: NamedAPIResource<GrowthRate>,
    pub pokedex_numbers: Vec<PokemonSpeciesDexEntry>,
    pub egg_groups: Vec<NamedAPIResource<EggGroup>>,
    pub color: NamedAPIResource<PokemonColor>,
    pub shape: NamedAPIResource<PokemonShape>,
    pub evolves_from_species: Option<NamedAPIResource<PokemonSpecies>>,
    pub evolution_chain: APIResource<EvolutionChain>,
    pub habitat: Option<NamedAPIResource<PokemonHabitat>>,
    pub generation: NamedAPIResource<Generation>,
    pub names: Vec<Name>,
    pub pal_park_encounters: Vec<PalParkEncounterArea>,
    pub flavor_text_entries: Vec<FlavorText>,
    pub form_descriptions: Vec<Description>,
    pub genera: Vec<Genus>,
    pub varieties: Vec<PokemonSpeciesVariety>,
}

/// <https://pokeapi.co/docs/v2.html#genus>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Genus {
    pub genus: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonspeciesdexentry>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonSpeciesDexEntry {
    pub entry_number: u16,
    pub pokedex: NamedAPIResource<Pokedex>,
}

/// <https://pokeapi.co/docs/v2.html#palparkencounterarea>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PalParkEncounterArea {
    pub base_score: u8,
    pub rate: u8,
    pub area: NamedAPIResource<PalParkArea>,
}

/// <https://pokeapi.co/docs/v2.html#pokemonspeciesvariety>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonSpeciesVariety {
    pub is_default: bool,
    pub pokemon: NamedAPIResource<Pokemon>,
}

/// <https://pokeapi.co/docs/v2.html#stats>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Stat {
    pub id: i16,
    pub name: String,
    pub game_index: i16,
    pub is_battle_only: bool,
    pub affecting_moves: MoveStatAffectSets,
    pub affecting_natures: NatureStatAffectSets,
    pub characteristics: Vec<APIResource<Characteristic>>, // incorrectly documented as APIResource
    pub move_damage_class: Option<NamedAPIResource<MoveDamageClass>>,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#movestataffectsets>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveStatAffectSets {
    pub increase: Vec<MoveStatAffect>,
    pub decrease: Vec<MoveStatAffect>,
}

/// <https://pokeapi.co/docs/v2.html#movestataffect>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveStatAffect {
    pub change: i8,
    #[serde(rename = "move")]
    pub move_: NamedAPIResource<Move>,
}

/// <https://pokeapi.co/docs/v2.html#naturestataffectsets>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NatureStatAffectSets {
    pub increase: Vec<NamedAPIResource<MoveStatAffect>>,
    pub decrease: Vec<NamedAPIResource<MoveStatAffect>>,
}

/// <https://pokeapi.co/docs/v2.html#types>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Type {
    pub id: i16,
    pub name: String,
    pub damage_relations: TypeRelations,
    pub game_indices: Vec<GenerationGameIndex>,
    pub generation: NamedAPIResource<Generation>,
    pub move_damage_class: Option<NamedAPIResource<MoveDamageClass>>,
    pub names: Vec<Name>,
    pub pokemon: Vec<TypePokemon>,
    pub moves: Vec<NamedAPIResource<Move>>,
}

/// <https://pokeapi.co/docs/v2.html#typepokemon>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct TypePokemon {
    pub slot: u8,
    pub pokemon: NamedAPIResource<Pokemon>,
}

/// <https://pokeapi.co/docs/v2.html#typerelations>
////#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct TypeRelations {
    pub no_damage_to: Vec<NamedAPIResource<Type>>,
    pub half_damage_to: Vec<NamedAPIResource<Type>>,
    pub double_damage_to: Vec<NamedAPIResource<Type>>,
    pub no_damage_from: Vec<NamedAPIResource<Type>>,
    pub half_damage_from: Vec<NamedAPIResource<Type>>,
    pub double_damage_from: Vec<NamedAPIResource<Type>>,
}

set_endpoint!(Ability, NamedAPIResourceList, "ability");
set_endpoint!(Characteristic, APIResourceList, "characteristic");
set_endpoint!(EggGroup, NamedAPIResourceList, "egg-group");
set_endpoint!(Gender, NamedAPIResourceList, "gender");
set_endpoint!(GrowthRate, NamedAPIResourceList, "growth-rate");
set_endpoint!(Nature, NamedAPIResourceList, "nature");
set_endpoint!(PokeathlonStat, NamedAPIResourceList, "pokeathlon-stat");
set_endpoint!(Pokemon, NamedAPIResourceList, "pokemon");
set_endpoint!(PokemonColor, NamedAPIResourceList, "pokemon-color");
set_endpoint!(PokemonForm, NamedAPIResourceList, "pokemon-form");
set_endpoint!(PokemonHabitat, NamedAPIResourceList, "pokemon-habitat");
set_endpoint!(PokemonShape, NamedAPIResourceList, "pokemon-shape");
set_endpoint!(PokemonSpecies, NamedAPIResourceList, "pokemon-species");
set_endpoint!(Stat, NamedAPIResourceList, "stat");
set_endpoint!(Type, NamedAPIResourceList, "type");

impl_id!(Characteristic);
impl_id_and_named!(Ability);
impl_id_and_named!(EggGroup);
impl_id_and_named!(Gender);
impl_id_and_named!(GrowthRate);
impl_id_and_named!(Nature);
impl_id_and_named!(PokeathlonStat);
impl_id_and_named!(Pokemon);
impl_id_and_named!(PokemonColor);
impl_id_and_named!(PokemonForm);
impl_id_and_named!(PokemonHabitat);
impl_id_and_named!(PokemonShape);
impl_id_and_named!(PokemonSpecies);
impl_id_and_named!(Stat);
impl_id_and_named!(Type);
