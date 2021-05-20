// #![feature(exclusive_range_pattern)]

use std::path::Path;
use std::sync::Arc;

use firecore_pokedex::pokemon::types::PokemonType;
use tokio::task;

mod entries;
mod images;
mod moves;

pub(crate) const EXTENSION: &str = "ron";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    /*
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("m", "no-moves", "Disable move generation");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(err) => {panic!("{}", err)}
    };
    */

    let start = tokio::time::Instant::now();

    let path = Path::new("pokedex/");

    if !path.exists() {
        tokio::fs::create_dir(path).await?;
    }

    let client = Arc::new(reqwest::Client::builder().build()?);
    let client_ = client.clone();

    let entries_task = task::spawn(async move {
        entries::add_entries(client_).await.unwrap();
    });    
    
    let moves_task = task::spawn(async move {
        moves::add_moves(client).await.unwrap();
    });

    entries_task.await?;
    moves_task.await?;

    let elapsed = start.elapsed().as_millis() as f64 / 1000.0;
    
    println!("Finished in {} seconds!", elapsed);

    Ok(())
}

#[inline]
pub(crate) fn capitalize_first(string: &mut String) {
    string[..1].make_ascii_uppercase();
}



pub(crate) fn type_from_string(string: &str) -> PokemonType {
    match string[..3].to_ascii_lowercase().as_str() {
        "nor" => PokemonType::Normal,
        "fir" => PokemonType::Fire,
        "wat" => PokemonType::Water,
        "ele" => PokemonType::Electric,
        "gra" => PokemonType::Grass,
        "ice" => PokemonType::Ice,
        "fig" => PokemonType::Fighting,
        "poi" => PokemonType::Poison,
        "gro" => PokemonType::Ground,
        "fly" => PokemonType::Flying,
        "psy" => PokemonType::Psychic,
        "bug" => PokemonType::Bug,
        "roc" => PokemonType::Rock,
        "gho" => PokemonType::Ghost,
        "dra" => PokemonType::Dragon,
        "dar" => PokemonType::Dark,
        "ste" => PokemonType::Steel,
        "fai" => PokemonType::Fairy,
        _ => panic!("Could not get pokemon type from string \"{}\"", string),
    }
}