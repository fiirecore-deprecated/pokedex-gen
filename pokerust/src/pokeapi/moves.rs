use serde::{Deserialize, Serialize};

use super::contests::*;
use super::games::*;
use super::pokemon::AbilityEffectChange;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#moves>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Move {
    pub id: i16,
    pub name: String,
    pub accuracy: Option<u8>,
    pub effect_chance: Option<u8>,
    pub pp: Option<u8>,
    pub priority: i8,
    pub power: Option<u8>,
    pub contest_combos: Option<ContestComboSets>,
    pub contest_type: Option<NamedAPIResource<ContestType>>,
    pub contest_effect: Option<APIResource<ContestEffect>>,
    pub damage_class: NamedAPIResource<MoveDamageClass>,
    pub effect_entries: Vec<VerboseEffect>,
    pub effect_changes: Vec<AbilityEffectChange>,
    pub flavor_text_entries: Vec<MoveFlavorText>,
    pub generation: NamedAPIResource<Generation>,
    pub machines: Vec<MachineVersionDetail>,
    pub meta: Option<MoveMetaData>,
    pub names: Vec<Name>,
    pub past_values: Vec<PastMoveStatValues>,
    pub stat_changes: Vec<MoveStatChange>,
    pub super_contest_effect: Option<APIResource<SuperContestEffect>>,
    pub target: NamedAPIResource<MoveTarget>,
    #[serde(rename = "type")]
    pub type_: NamedAPIResource<Type>,
}

/// <https://pokeapi.co/docs/v2.html#contestcombosets>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ContestComboSets {
    pub normal: ContestComboDetail,
    #[serde(rename = "super")]
    pub super_: ContestComboDetail,
}

/// <https://pokeapi.co/docs/v2.html#contestcombodetail>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ContestComboDetail {
    pub use_before: Option<Vec<NamedAPIResource<Move>>>,
    pub use_after: Option<Vec<NamedAPIResource<Move>>>,
}

/// <https://pokeapi.co/docs/v2.html#moveflavortext>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveFlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource<Language>, // incorrectly documented as list NamedAPIResource (Move)
    pub version_group: NamedAPIResource<VersionGroup>, // incorrectly documented as list NamedAPIResource (Move)
}

/// <https://pokeapi.co/docs/v2.html#movemetadata>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveMetaData {
    pub ailment: NamedAPIResource<MoveAilment>,
    pub category: NamedAPIResource<MoveCategory>, // incorrectly documented as NamedApiResource (Move)
    pub min_hits: Option<u8>,
    pub max_hits: Option<u8>,
    pub min_turns: Option<u8>,
    pub max_turns: Option<u8>,
    pub drain: i8,
    pub healing: i8,
    pub crit_rate: u8,
    pub ailment_chance: u8,
    pub flinch_chance: u8,
    pub stat_chance: u8,
}

/// <https://pokeapi.co/docs/v2.html#movestatchange>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveStatChange {
    pub change: i8,
    pub stat: NamedAPIResource<Stat>,
}

/// <https://pokeapi.co/docs/v2.html#pastmovestatvalues>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PastMoveStatValues {
    pub accuracy: Option<u8>,
    pub effect_chance: Option<u8>,
    pub power: Option<u8>,
    pub pp: Option<u8>,
    pub effect_entries: Vec<VerboseEffect>,
    #[serde(rename = "type")]
    type_: Option<NamedAPIResource<Type>>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

/// <https://pokeapi.co/docs/v2.html#move-ailments>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveAilment {
    pub id: i16,
    pub name: String,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#move-battle-styles>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveBattleStyle {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
}

// incorrectly documeted as ModelName
/// <https://pokeapi.co/docs/v2.html#move-categories>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveCategory {
    pub id: i16,
    pub name: String,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub descriptions: Vec<Description>,
}

/// <https://pokeapi.co/docs/v2.html#move-damage-classes>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveDamageClass {
    pub id: i16,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#move-learn-methods>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveLearnMethod {
    pub id: i16,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub names: Vec<Name>,
    pub version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

/// <https://pokeapi.co/docs/v2.html#move-targets>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MoveTarget {
    pub id: i16,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub names: Vec<Name>,
}

set_endpoint!(Move, NamedAPIResourceList, "move");
set_endpoint!(MoveAilment, NamedAPIResourceList, "move-ailment");
set_endpoint!(MoveBattleStyle, NamedAPIResourceList, "move-battle-style");
set_endpoint!(MoveCategory, NamedAPIResourceList, "move-category");
set_endpoint!(MoveDamageClass, NamedAPIResourceList, "move-damage-class");
set_endpoint!(MoveLearnMethod, NamedAPIResourceList, "move-learn-method");
set_endpoint!(MoveTarget, NamedAPIResourceList, "move-target");

impl_id_and_named!(Move);
impl_id_and_named!(MoveAilment);
impl_id_and_named!(MoveBattleStyle);
impl_id_and_named!(MoveCategory);
impl_id_and_named!(MoveDamageClass);
impl_id_and_named!(MoveLearnMethod);
impl_id_and_named!(MoveTarget);
