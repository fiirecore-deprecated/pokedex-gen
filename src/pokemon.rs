use battle::pokedex::{
    pokemon::{
        data::{Breeding, GrowthRate, LearnableMove, Training},
        stat::{StatSet, StatType},
        Pokemon, PokemonTexture, PokemonId
    },
    types::Types,
};

use enum_map::{EnumMap, enum_map};
use pokerust::Id;
use hashbrown::HashMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;
use std::sync::Arc;
use tempfile::TempDir;

use crate::capitalize_first;

pub type SerializedPokemon = (EnumMap<PokemonTexture, Vec<u8>>, Vec<u8>);
pub type PokemonOutput = HashMap<PokemonId, SerializedPokemon>;

mod cry;
mod images;

pub use images::download;

const DEX_SIZE: i16 = 386;

const FRONT: &str = "front";
const BACK: &str = "back";
const ICON: &str = "icon";

#[cfg(windows)]
const FFMPEG_PATH: &str = "ffmpeg.exe";

#[cfg(not(windows))]
const FFMPEG_PATH: &str = "ffmpeg";

pub fn add_pokemon(
    tempdir: Arc<TempDir>,
    pokerust: Arc<pokerust::Client>,
) -> (Vec<Pokemon>, PokemonOutput) {
    let enable_cry = Path::new(FFMPEG_PATH).exists();

    let vec = (1..DEX_SIZE)
        .into_par_iter()
        .map(|index| {
            let pokerust = pokerust.clone();
            let tempdir = tempdir.clone();
            let (p, s) = get_pokemon(index, &pokerust, tempdir, enable_cry);
            let s = (p.id, s);
            (p, s)
        })
        .unzip();

    vec
}

fn get_pokemon(
    index: i16,
    pokerust: &pokerust::Client,
    tempdir: Arc<TempDir>,
    enable_cry: bool,
) -> (Pokemon, SerializedPokemon) {
    // let before_move_check = start.elapsed().as_micros();

    let pokemon: pokerust::Pokemon = pokerust
        .get(index)
        .unwrap_or_else(|err| panic!("Could not get pokemon at {} with error {}", index, err));

    let mut name = pokemon.name.clone();

    capitalize_first(&mut name);

    println!("Creating pokemon entry for: {}", name);

    let mut name_ = pokemon.name;
    unsafe {
        let find = '-' as u8;
        let replace = '_' as u8;
        name_.as_bytes_mut().iter_mut().for_each(|u| {
            if *u == find {
                *u = replace;
            }
        })
    }
    match &name_[..2] {
        "un" => name_.push_str("/e"),

        _ => (),
    };

    let name_counted = Arc::new(name_);

    let nc = name_counted.clone();

    let cry = std::thread::spawn(move || {
        enable_cry
            .then(|| cry::get_cry(tempdir, nc))
            .unwrap_or_default()
    });

    let mut textures = [FRONT, BACK, ICON]
        .into_par_iter()
        .map(move |side| download(name_counted.clone(), side))
        .collect::<Vec<_>>();

    // let after_move_check = start.elapsed().as_micros();

    let primary = crate::type_from_id(pokemon.types[0].type_.id());
    let secondary = if pokemon.types.len() == 2 {
        Some(crate::type_from_id(pokemon.types[1].type_.id()))
    } else {
        None
    };

    let species: pokerust::PokemonSpecies = pokerust.get(pokemon.species.id()).unwrap();
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

    // let evolution: EvolutionChain = pokerust.get(species.evolution_chain.id()).await.unwrap();

    // evolution.chain.species

    let cry = cry.join().unwrap();

    (
        Pokemon {
            id: pokemon.id as u16,
            name,
            types: Types { primary, secondary },
            moves,
            base: StatSet(enum_map! {
                StatType::Health => stats[0].base_stat,
                StatType::Attack => stats[1].base_stat,
                StatType::Defense => stats[2].base_stat,
                StatType::SpAttack => stats[3].base_stat,
                StatType::SpDefense => stats[4].base_stat,
                StatType::Speed => stats[5].base_stat,
            }),
            species: genus,
            evolution: None,
            height: pokemon.height,
            weight: pokemon.weight,
            training: Training {
                base_exp: pokemon.base_experience,
                growth: growth_rate_from_id(species.growth_rate.id()),
            },
            breeding: Breeding {
                gender: match species.gender_rate {
                    0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => Some(species.gender_rate as u8),
                    _ => None,
                },
            },
        },
        (
            EnumMap::from_array([textures.remove(0), textures.remove(0), textures.remove(0)]),
            cry,
        ),
    )
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
