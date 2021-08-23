extern crate firecore_battle as battle;
// #![feature(exclusive_range_pattern)]

use std::path::Path;
use std::sync::Arc;

use log::info;
use battle::pokedex::types::PokemonType;
use tokio::task;

mod pokemon;
mod moves;

// pub(crate) const EXTENSION: &str = "ron";

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;

    let start = tokio::time::Instant::now();

    let path = Path::new("pokedex/");

    if !path.exists() {
        tokio::fs::create_dir(path).await?;
    }

    let client = Arc::new(pokerust::Client::default());
    let client_ = client.clone();

    let pokemon_task = task::spawn(async move {
        pokemon::add_pokemon(client_).await.unwrap();
    });    
    
    let moves_task = task::spawn(async move {
        moves::add_moves(client).await.unwrap();
    });

    pokemon_task.await?;
    moves_task.await?;

    let elapsed = start.elapsed().as_millis() as f64 / 1000.0;
    
    info!("Finished in {} seconds!", elapsed);

    Ok(())
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