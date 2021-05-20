use std::path::Path;
use std::sync::Arc;

use firecore_pokedex::moves::Move;
use firecore_pokedex::moves::MoveCategory;
use pokerust::FromId;
use tokio::io::AsyncWriteExt;

const MOVES_SIZE: i16 = 559;

const MOVES_PATH: &str = "pokedex/moves/";

pub async fn add_moves(client: Arc<reqwest::Client>) -> Result<(), Box<dyn std::error::Error>> {

    let path = Path::new(MOVES_PATH);
    if !path.exists() {
        tokio::fs::create_dir(&path).await?;
    }

    for index in 1..MOVES_SIZE {
        let client = client.clone();
        tokio::task::spawn(async move {
            get_move(index, client.as_ref(), &path).await.unwrap();
        });
    }

    Ok(())

}

async fn get_move(index: i16, client: &reqwest::Client, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut move_ = pokerust::Move::from_id(client, index).await?;

    let name = move_.names.remove(7).name;

    let id = move_.name.parse().expect("Could not parse move name into ASCII string!");

    crate::capitalize_first(&mut move_.type_.name);
    crate::capitalize_first(&mut move_.damage_class.name);

    println!("Creating move entry for: {}", name);

    let mut file = tokio::fs::File::create(path.join(format!("{}.{}", name, crate::EXTENSION))).await?;
    file.write_all(ron::ser::to_string_pretty(&Move {
        id,
        pp: move_.pp.unwrap_or_else(|| panic!("Could not get PP for pokemon move {}", name)),
        name,
        category: category_from_string(&move_.damage_class.name),
        pokemon_type: crate::type_from_string(&move_.type_.name),
        power: move_.power,
        accuracy: move_.accuracy,
        target: firecore_pokedex::moves::target::MoveTarget::Opponent,
        script: None,

    }, ron::ser::PrettyConfig::default())?.as_bytes()).await?;

    Ok(())
}

fn category_from_string(string: &str) -> MoveCategory {
    match string[..2].to_ascii_lowercase().as_str() {
        "ph" => MoveCategory::Physical,
        "sp" => MoveCategory::Special,
        "st" => MoveCategory::Status,
        _ => panic!("Could not get move category from string \"{}\"", string),
    }
}