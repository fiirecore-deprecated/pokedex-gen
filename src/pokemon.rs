use std::path::Path;
use std::sync::Arc;
use log::info;
use pokedex::pokemon::{
    Pokemon,
    data::{
        LearnableMove,
        PokedexData,
        GrowthRate,
        Training,
        Breeding,
    },
    stat::StatSet,
};
use pokerust::Id;
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

pub async fn add_entries(client: Arc<pokerust::Client>) -> anyhow::Result<()> {

    let entry_path = Path::new(ENTRY_PATH);
    if !entry_path.exists() {
        tokio::fs::create_dir(entry_path).await?;
    }

    for index in 1..DEX_SIZE {
        let client = client.clone();
        tokio::spawn(async move { 
            get_pokemon(index, &client, &entry_path).await.unwrap() 
        }).await.unwrap();
    }
        
    
    Ok(())

}

async fn get_pokemon(index: i16, client: &pokerust::Client, entry_path: &Path) -> anyhow::Result<()> {

    // let before_move_check = start.elapsed().as_micros();

    let mut pokemon: pokerust::Pokemon = client.get(index).await?;

    info!("Creating pokemon entry for: {}", &pokemon.name);

    let folder = entry_path.join(&pokemon.name);

    if !folder.exists() {
        tokio::fs::create_dir_all(&folder).await?;
    }

    let folder_ = folder.clone();
    let name_ = Arc::new(pokemon.name.clone());
    let index_ = index;

    if index_ < 152 {

        let name = name_.clone();
        let folder = folder_.clone();
        tokio::spawn(async move { ImageWriter::download(&folder, IMG_PATH1, name.as_ref(), FRONT, FRONT_URL).await });

        let name = name_.clone();
        let folder = folder_.clone();
        tokio::spawn(async move { ImageWriter::download(&folder, IMG_PATH1, name.as_ref(), BACK, BACK_URL).await });
    } else {

        let name = name_.clone();
        let folder = folder_.clone();
        tokio::spawn(async move { ImageWriter::download(&folder, IMG_PATH2, name.as_ref(), FRONT, FRONT_URL).await });

        let name = name_;
        let folder = folder_;
        tokio::spawn(async move { ImageWriter::download(&folder, IMG_PATH2, name.as_ref(), BACK, BACK_URL).await });
    }
    
    // let after_move_check = start.elapsed().as_micros();

    capitalize_first(&mut pokemon.name);

    let primary_type = crate::type_from_id(pokemon.types[0].type_.id());
    let secondary_type = if pokemon.types.len() == 2 {
        Some(crate::type_from_id(pokemon.types[1].type_.id()))
    } else {
        None
    };


    let species: pokerust::PokemonSpecies = client.get(pokemon.species.id()).await?;
    let genus = &species.genera[7].genus;
    let genus = genus[0..genus.find(" ").unwrap_or(genus.len() - 1)].to_string();

    // Stats

    let stats = &pokemon.stats;

    let mut moves = Vec::new();

    for pmove in &pokemon.moves {
        for version in &pmove.version_group_details {
            if version.version_group.name.starts_with("f") && version.level_learned_at != 0  {
                moves.push(LearnableMove {
                    id: pmove.move_.name.parse().expect("Could not parse learnable move id!"),
                    level: version.level_learned_at,
                });
            }
        }
    }

    let mut file = tokio::fs::File::create(folder.join("pokemon.ron")).await?;
    file.write_all(ron::ser::to_string_pretty(&Pokemon {
        id: pokemon.id as u16,
        name: pokemon.name,
        primary_type,
        secondary_type,
        data: PokedexData {
            species: genus,
            height: pokemon.height,
            weight: pokemon.weight,
        },
        training: Training {
            base_exp: pokemon.base_experience,
            growth_rate: growth_rate_from_id(species.growth_rate.id()),
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

    Ok(())

}

pub fn growth_rate_from_id(id: i16) -> GrowthRate {
    match id {
        1 => GrowthRate::Slow,
        2 => GrowthRate::Medium,
        3 => GrowthRate::Fast,
        4 => GrowthRate::MediumSlow,
        6 => GrowthRate::FastThenVerySlow,
        5 => GrowthRate::SlowThenVeryFast,
        _ => panic!("Could not get growth rate from id \"{}\"", id)
    }
}