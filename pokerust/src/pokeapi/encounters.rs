use serde::{Deserialize, Serialize};

use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#encounter-methods>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct EncounterMethod {
    pub id: i16,
    pub name: String,
    pub order: u16,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#encounter-conditions>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct EncounterCondition {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub values: Vec<NamedAPIResource<EncounterConditionValue>>,
}

/// <https://pokeapi.co/docs/v2.html#encounter-condition-values>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct EncounterConditionValue {
    pub id: i16,
    pub name: String,
    pub condition: NamedAPIResource<EncounterCondition>, // incorrectly documented as list NamedAPIResource
    pub names: Vec<Name>,
}

set_endpoint!(EncounterMethod, NamedAPIResourceList, "encounter-method");
set_endpoint!(
    EncounterCondition,
    NamedAPIResourceList,
    "encounter-condition"
);
set_endpoint!(
    EncounterConditionValue,
    NamedAPIResourceList,
    "encounter-condition-value"
);

impl_id_and_named!(EncounterMethod);
impl_id_and_named!(EncounterCondition);
impl_id_and_named!(EncounterConditionValue);
