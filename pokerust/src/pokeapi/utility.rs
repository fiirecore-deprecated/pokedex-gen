use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{impl_id_and_named, impl_named, set_endpoint, Id, Named};

use super::encounters::*;
use super::games::*;
use super::machines::*;
use super::resource_lists::*;

use std::marker::PhantomData;

// Extract the id from a url containing one, e.g. https://pokeapi.co/api/v2/item/38/
fn id_from_url(url: &str) -> i16 {
    let url = &url[..(url.len() - 1)];
    url[(url.rfind('/').unwrap() + 1)..].parse().unwrap()
}

/// <https://pokeapi.co/docs/v2.html#languages>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Language {
    pub id: i16,
    pub name: String,
    pub official: bool,
    pub iso639: String,
    pub iso3166: String,
    pub names: Vec<Name>,
}

/// <https://pokeapi.co/docs/v2.html#apiresource>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct APIResource<T> {
    pub url: String,
    #[serde(skip)]
    resource_type: PhantomData<*const T>,
}

impl<T> Id for APIResource<T> {
    fn id(&self) -> i16 {
        id_from_url(&self.url)
    }
}

impl<T> APIResource<T>
where
    T: DeserializeOwned,
{
    pub fn get(&self, client: &crate::Client) -> Result<T, attohttpc::Error> {
        client.get_api_loc(&self.url)
    }
}

unsafe impl<T> Send for APIResource<T> {}
unsafe impl<T> Sync for APIResource<T> {}

/// <https://pokeapi.co/docs/v2.html#description>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Description {
    pub description: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#effect>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Effect {
    pub effect: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#encounter>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Encounter {
    pub min_level: u8,
    pub max_level: u8,
    pub condition_values: Vec<NamedAPIResource<EncounterConditionValue>>,
    pub chance: u8,
    pub method: NamedAPIResource<EncounterMethod>,
}

/// <https://pokeapi.co/docs/v2.html#flavortext>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource<Language>,
    pub version: Option<NamedAPIResource<Version>>, // sometimes this isn't provided, e.g. https://pokeapi.co/api/v2/contest-effect/9/
}

/// <https://pokeapi.co/docs/v2.html#generationgameindex>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GenerationGameIndex {
    pub game_index: i16,
    pub generation: NamedAPIResource<Generation>,
}

/// <https://pokeapi.co/docs/v2.html#machineversiondetail>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct MachineVersionDetail {
    pub machine: APIResource<Machine>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

/// <https://pokeapi.co/docs/v2.html#name>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#namedapiresource>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct NamedAPIResource<T> {
    pub name: String,
    pub url: String,
    #[serde(skip)]
    resource_type: PhantomData<*const T>,
}

impl<T> Id for NamedAPIResource<T> {
    fn id(&self) -> i16 {
        id_from_url(&self.url)
    }
}

impl<T> Named for NamedAPIResource<T> {
    fn name(&self) -> &String {
        &self.name
    }
}

impl<T> NamedAPIResource<T>
where
    T: DeserializeOwned,
{
    pub fn get(&self, client: &crate::Client) -> Result<T, attohttpc::Error> {
        client.get_api_loc(&self.url)
    }
}

unsafe impl<T> Send for NamedAPIResource<T> {}
unsafe impl<T> Sync for NamedAPIResource<T> {}

/// <https://pokeapi.co/docs/v2.html#verboseeffect>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct VerboseEffect {
    pub effect: String,
    pub short_effect: String,
    pub language: NamedAPIResource<Language>,
}

/// <https://pokeapi.co/docs/v2.html#versionencounterdetail>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct VersionEncounterDetail {
    pub version: NamedAPIResource<Version>,
    pub max_chance: u16,
    pub encounter_details: Vec<Encounter>,
}

/// <https://pokeapi.co/docs/v2.html#versiongameindex>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct VersionGameIndex {
    pub game_index: i16,
    pub version: NamedAPIResource<Version>,
}

/// <https://pokeapi.co/docs/v2.html#versiongroupflavortext>
//#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct VersionGroupFlavorText {
    pub text: String,
    pub language: NamedAPIResource<Language>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

set_endpoint!(Language, NamedAPIResourceList, "language");

impl_named!(Name);
impl_id_and_named!(Language);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_url() {
        assert_eq!(
            id_from_url("https://pokeapi.co/api/v2/move-ailment/-1/"),
            -1
        );
        assert_eq!(id_from_url("https://pokeapi.co/api/v2/move-category/0/"), 0);
        assert_eq!(id_from_url("http://localhost:8000/api/v2/item/38/"), 38);
    }
}
