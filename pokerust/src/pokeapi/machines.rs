use serde::{Deserialize, Serialize};

use super::games::*;
use super::items::*;
use super::moves::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id, set_endpoint};

/// <https://pokeapi.co/docs/v2.html#machines>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Machine {
    pub id: i16,
    pub item: NamedAPIResource<Item>,
    #[serde(rename = "move")]
    pub move_: NamedAPIResource<Move>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

set_endpoint!(Machine, APIResourceList, "machine");

impl_id!(Machine);
