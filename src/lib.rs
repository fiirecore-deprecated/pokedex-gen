extern crate firecore_battle as battle;
// #![feature(exclusive_range_pattern)]

use std::sync::Arc;

use battle::pokedex::{moves::Move, pokemon::Pokemon, types::PokemonType, item::Item};
use firecore_pokedex_engine_builder::pokemon::PokemonOutput;
use moves::Execution;
use serde::{Deserialize, Serialize};

mod moves;
mod pokemon;
mod items;

// pub(crate) const EXTENSION: &str = "ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct DexGenerator {
    pub pokemon: GeneratedPokemon,
    pub moves: GeneratedMoves,
    pub items: GeneratedItems,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedPokemon {
    pub pokemon: Vec<Pokemon>,
    pub ui_data: PokemonOutput,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedMoves {
    pub moves: Vec<Move>,
    pub execution: Execution,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedItems {
    pub items: Vec<Item>,
    pub textures: items::ItemTextures,
}

pub fn generate() -> DexGenerator {
    // std::env::set_var("SMOL_THREADS", &std::ffi::OsString::from("10"));

    let start = std::time::Instant::now();

    let tempdir =
        Arc::new(tempfile::TempDir::new().unwrap_or_else(|err| {
            panic!("Could not create temporary directory with error {}", err)
        }));

    let pokerust = Arc::new(pokerust::Client::default());

    let pokerust1 = pokerust.clone();
    let pokerust2 = pokerust.clone();
    
    // let client_ = client.clone();

    let pokemon_thread = std::thread::spawn(|| {
    pokemon::add_pokemon(tempdir, pokerust1)
    });

    let moves_thread = std::thread::spawn(|| {
        moves::add_moves(pokerust2)
    });

    let items_thread = std::thread::spawn(|| {
        items::add_items(pokerust)
    });

    let (moves, battle_moves) = moves_thread.join().unwrap(); //moves_thread.join().unwrap();

    let (pokemon, serpokemon) = pokemon_thread.join().unwrap(); //pokemon_thread.join().unwrap();

    let (items, item_textures) = items_thread.join().unwrap();

    let elapsed = start.elapsed().as_millis() as f64 / 1000.0;

    println!("Finished in {} seconds!", elapsed);

    DexGenerator {
        pokemon: GeneratedPokemon {
            pokemon,
            ui_data: serpokemon,
        },
        moves: GeneratedMoves {
            moves,
            execution: battle_moves,
        },
        items: GeneratedItems {
            items,
            textures: item_textures,
        }
    }
}

#[inline]
pub(crate) fn capitalize_first(string: &mut String) {
    string[..1].make_ascii_uppercase();
}

pub(crate) fn type_from_id(id: i16) -> PokemonType {
    match id {
        1 => PokemonType::Normal,
        10 => PokemonType::Fire,
        11 => PokemonType::Water,
        13 => PokemonType::Electric,
        12 => PokemonType::Grass,
        15 => PokemonType::Ice,
        2 => PokemonType::Fighting,
        4 => PokemonType::Poison,
        5 => PokemonType::Ground,
        3 => PokemonType::Flying,
        14 => PokemonType::Psychic,
        7 => PokemonType::Bug,
        6 => PokemonType::Rock,
        8 => PokemonType::Ghost,
        16 => PokemonType::Dragon,
        17 => PokemonType::Dark,
        9 => PokemonType::Steel,
        18 => PokemonType::Fairy,
        _ => panic!("Could not get pokemon type from id \"{}\"", id),
    }
}
