use anyhow::Result;
use battle::pokedex::pokemon::{
    stat::StatSet, Breeding, GrowthRate, LearnableMove, Pokemon, Training,
};
use log::info;
use pokerust::Id;
use std::path::Path;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::capitalize_first;

mod cry;
mod images;

pub use images::download;

const DEX_SIZE: i16 = 386;

const FRONT: &str = "front";
const BACK: &str = "back";
const ICON: &str = "icon";

const POKEMON_PATH: &str = "pokedex/pokemon/";
const CLIENT_POKEMON_PATH: &str = "pokedex/client/pokemon/";

#[cfg(windows)]
const FFMPEG_PATH: &str = "./ffmpeg.exe";

#[cfg(not(windows))]
const FFMPEG_PATH: &str = "./ffmpeg";

pub async fn add_pokemon(client: Arc<pokerust::Client>) -> Result<()> {
    let path = Path::new(POKEMON_PATH);

    if !path.exists() {
        tokio::fs::create_dir_all(path).await?;
    }

    let path = Path::new(CLIENT_POKEMON_PATH);

    if !path.exists() {
        tokio::fs::create_dir_all(path).await?;
    }

    let enable_cry = Path::new(FFMPEG_PATH).exists();

    for index in 1..DEX_SIZE {
        let client = client.clone();
        tokio::spawn(async move { get_pokemon(index, &client, enable_cry, &path).await.unwrap() })
            .await
            .unwrap();
    }

    Ok(())
}

async fn get_pokemon(
    index: i16,
    client: &pokerust::Client,
    enable_cry: bool,
    client_path: &Path,
) -> anyhow::Result<()> {
    // let before_move_check = start.elapsed().as_micros();

    let pokemon: pokerust::Pokemon = client.get(index).await?;

    let mut name = pokemon.name.clone();

    capitalize_first(&mut name);

    info!("Creating pokemon entry for: {}", name);

    let client_folder = client_path.join(&name);

    if !client_folder.exists() {
        tokio::fs::create_dir_all(&client_folder).await?;
    }

    let client_folder_ = client_folder.clone();
    let mut name_ = pokemon.name;
    unsafe {
        name_.as_bytes_mut().iter_mut().for_each(|u| {
            let find = '-' as u8;
            let replace = '_' as u8;
            if *u == find {
                *u = replace;
            }
        })
    }
    match &name_[..2] {
        "un" => name_.push_str("/e"),

        _ => (),
    };

    let name_ = Arc::new(name_);

    {
        let name = name_.clone();
        let client_folder = client_folder_.clone();
        tokio::spawn(async move { download(&client_folder, name.as_ref(), FRONT).await });

        let name = name_.clone();
        let client_folder = client_folder_.clone();
        tokio::spawn(async move { download(&client_folder, name.as_ref(), BACK).await });

        let name = name_.clone();
        let client_folder = client_folder_.clone();
        tokio::spawn(async move { download(&client_folder, name.as_ref(), ICON).await });
    }

    if enable_cry {

        let name = name_.clone();
        let client_folder = client_folder_.clone();
        tokio::spawn(async move { cry::get_cry(&client_folder, &name).await.unwrap_or_else(|err| panic!("Could not get cry with error {}", err)); });

    }

    // let after_move_check = start.elapsed().as_micros();

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
            if version.version_group.name.starts_with("f") && version.level_learned_at != 0 {
                moves.push(LearnableMove(
                    version.level_learned_at,
                    pmove
                        .move_
                        .name
                        .parse()
                        .expect("Could not parse learnable move id!"),
                ));
            }
        }
    }

    let mut file = tokio::fs::File::create(format!("{}{}.ron", POKEMON_PATH, name)).await?;
    file.write_all(
        ron::ser::to_string_pretty(
            &Pokemon {
                id: pokemon.id as u16,
                name,
                primary_type,
                secondary_type,
                species: genus,
                height: pokemon.height,
                weight: pokemon.weight,
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
                        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => Some(species.gender_rate as u8),
                        _ => None,
                    },
                },
                moves,
            },
            ron::ser::PrettyConfig::default(),
        )?
        .as_bytes(),
    )
    .await?;

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
        _ => panic!("Could not get growth rate from id \"{}\"", id),
    }
}
