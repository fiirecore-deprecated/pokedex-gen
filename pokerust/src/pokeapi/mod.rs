pub mod berries;
pub mod contests;
pub mod encounters;
pub mod evolution;
pub mod games;
pub mod items;
pub mod locations;
pub mod machines;
pub mod moves;
pub mod pokemon;
pub mod resource_lists;
pub mod utility;

trait FromId {
    fn from_id(id: &u64) -> Self;
}

/// Gets the location of an API resource from a full url, minus the url
/// and common prefix, e.g. "https://pokeapi.co/api/v2/"
fn get_api_loc_from_url(url: &str) -> &str {
    let pre = "api/v2/";
    &url[(url.rfind(pre).unwrap() + pre.len())..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_loc_from_url() {
        assert_eq!(
            get_api_loc_from_url("https://pokeapi.co/api/v2/ability/?offset=20&limit=20"),
            "ability/?offset=20&limit=20"
        );
        assert_eq!(
            get_api_loc_from_url("http://localhost:8000/api/v2/pokemon/?limit=0&offset=42"),
            "pokemon/?limit=0&offset=42"
        );
        assert_eq!(
            get_api_loc_from_url("https://pokeapi.co/api/v2/api/v2/ability/?offset=20&limit=20"),
            "ability/?offset=20&limit=20"
        );
        assert_eq!(
            get_api_loc_from_url("https://pokeapi.co/api/v2/pokemon/25/encounters"),
            "pokemon/25/encounters"
        )
    }
}
