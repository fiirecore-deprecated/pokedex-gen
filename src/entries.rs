use std::path::Path;
use std::sync::Arc;
use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::pokemon::data::LearnableMove;
use firecore_pokedex::pokemon::data::PokedexData;
use firecore_pokedex::pokemon::data::training::GrowthRate;
use firecore_pokedex::pokemon::data::training::Training;
use firecore_pokedex::pokemon::stat::StatSet;
use firecore_pokedex::pokemon::data::breeding::Breeding;
use pokerust::Id;
use pokerust::FromId;
use tokio::io::AsyncWriteExt;

use crate::capitalize_first;
use crate::images::ImageWriter;

const DEX_SIZE: i16 = 386;

const IMG_PATH1: &str = "firered-leafgreen";
const IMG_PATH2: &str = "ruby-sapphire";
const FRONT_URL: &str = "normal";
const BACK_URL: &str = "back-normal";
const FRONT: &str = "front";
const BACK: &str = "back";

const ENTRY_PATH: &str = "pokedex/entries";

pub async fn add_entries(client: Arc<reqwest::Client>) -> Result<(), Box<dyn std::error::Error>> {

    let texture_path = Path::new("pokedex/textures");

    let entry_path = Path::new(ENTRY_PATH);
    if !entry_path.exists() {
        tokio::fs::create_dir(entry_path).await?;
    }

    if !texture_path.exists() {
        tokio::fs::create_dir_all(texture_path.join("normal/back")).await?;
        tokio::fs::create_dir_all(texture_path.join("normal/front")).await?;
    }

    for index in 1..DEX_SIZE {
        let client = client.clone();
        tokio::spawn(async move { 
            get_pokemon(index, &client, &entry_path).await.unwrap() 
        }).await.unwrap();
    }
        
    
    Ok(())

}

async fn get_pokemon(index: i16, client: &reqwest::Client, entry_path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    // let before_move_check = start.elapsed().as_micros();

    let pokemon = pokerust::Pokemon::from_id(client, index).await?;
    
    // let after_move_check = start.elapsed().as_micros();

    let mut name = pokemon.name.clone();

    capitalize_first(&mut name);

    let primary_type = crate::type_from_string(&pokemon.types[0].type_.name);
    let secondary_type = if pokemon.types.len() == 2 {
        Some(crate::type_from_string(&pokemon.types[1].type_.name))
    } else {
        None
    };


    let species = pokerust::PokemonSpecies::from_id(client, pokemon.species.id()).await?;
    let genus = &species.genera[7].genus;
    let genus = genus[0..genus.find(" ").unwrap_or(genus.len() - 1)].to_string();

    // Stats

    let stats = &pokemon.stats;

    let mut moves = Vec::new();

    for pmove in &pokemon.moves {
        for version in &pmove.version_group_details {
            if version.version_group.name.starts_with("f") && version.level_learned_at != 0  {
                moves.push(LearnableMove {
                    move_id: pmove.move_.name.parse().expect("Could not parse learnable move id!"),
                    level: version.level_learned_at,
                });
            }
        }
    }

    println!("Creating pokemon entry for: {}", &name);

    let folder = entry_path.join(&name);

    if !folder.exists() {
        tokio::fs::create_dir_all(&folder).await?;
    }

    let mut file = tokio::fs::File::create(folder.join(name.clone() + "." + crate::EXTENSION)).await?;
    file.write_all(ron::ser::to_string_pretty(&Pokemon {
        data: PokedexData {
            id: pokemon.id as u16,
            name: name,
            primary_type,
            secondary_type,
            species: genus,
            height: pokemon.height,
            weight: pokemon.weight,
        },
        training: Training {
            base_exp: pokemon.base_experience,
            growth_rate: growth_rate_from_string(&species.growth_rate.name),
        },
        base: StatSet {
            hp: stats[0].base_stat,
            atk: stats[1].base_stat,
            def: stats[2].base_stat,
            sp_atk: stats[3].base_stat,
            sp_def: stats[4].base_stat,
            speed: stats[5].base_stat,
        },
        breeding: Breeding {
            gender: match species.gender_rate {
                0|1|2|3|4|5|6|7|8|9 => Some(species.gender_rate as u8),
                _ => None,
            }
        },
        moves,
    }, ron::ser::PrettyConfig::default())?.as_bytes()).await?;
    

    if index < 152 {
        ImageWriter::download(IMG_PATH1, &pokemon.name, FRONT, FRONT_URL).await?;
        ImageWriter::download(IMG_PATH1, &pokemon.name, BACK, BACK_URL).await?;
    } else {
        ImageWriter::download(IMG_PATH2, &pokemon.name, FRONT, FRONT_URL).await?;
        ImageWriter::download(IMG_PATH2, &pokemon.name, BACK, BACK_URL).await?;
    }

    Ok(())

}

pub fn growth_rate_from_string(string: &str) -> GrowthRate {
    match string {
        "slow" => GrowthRate::Slow,
        "fast" => GrowthRate::Fast,
        "medium" => GrowthRate::Medium,
        "medium-slow" => GrowthRate::MediumSlow,
        "fast-then-very-slow" => GrowthRate::FastThenVerySlow,
        "slow-then-very-fast" => GrowthRate::SlowThenVeryFast,
        _ => panic!("Could not get growth rate from string \"{}\"", string)
    }
}

// #[derive(Serialize)]
// pub struct PokemonToml<'a> {

//     data: PokedexData<'a>,
//     training: Training<'a>,
//     base: StatSet,
//     breeding: Breeding,
//     moves: Vec<LearnableMove>,

// }

// #[derive(Serialize)]
// struct PokedexData<'a> {

//     number: &'a i16,
//     name: &'a String,
//     primary_type: String,
//     secondary_type: Option<String>,
//     species: &'a str,
//     height: &'a u8,
//     weight: &'a u16,

// }

// #[derive(Serialize)]
// pub struct Training<'a> {
//     base_exp: u16,
//     growth_rate: &'a String,
// }