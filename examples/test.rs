use std::path::Path;

use firecore_battle::pokedex::pokemon::PokemonTexture;
use ron::ser::PrettyConfig;

fn main() {
    let generated = firecore_dex_gen::generate();

    println!("Pokemon: {}", generated.pokemon.pokemon.len());

    println!("DexEngine: {}", generated.pokemon.ui_data.len());

    println!("Moves: {}", generated.moves.moves.len());

    println!("Battle Moves: {}", generated.moves.execution.len());

    std::fs::create_dir_all("generated/pokemon").unwrap();

    let pokemon = Path::new("generated/client/pokemon");

    std::fs::create_dir_all(pokemon).unwrap();

    for (index, (textures, cry)) in generated.pokemon.ui_data.into_iter() {
        // to - do: named folders
        let folder = format!("{}", index);

        let path = pokemon.join(folder);

        if !path.exists() {
            std::fs::create_dir(&path).unwrap();
        }

        for (texture, bytes) in textures.into_iter() {
            let file = match texture {
                PokemonTexture::Front => "front.png",
                PokemonTexture::Back => "back.png",
                PokemonTexture::Icon => "icon.png",
            };

            std::fs::write(path.join(file), bytes).unwrap();
        }

        if !cry.is_empty() {
            std::fs::write(path.join("cry.ogg"), cry).unwrap();
        }
    }

    for pokemon in generated.pokemon.pokemon.into_iter() {
        std::fs::write(
            format!("generated/pokemon/{}.ron", pokemon.name),
            ron::ser::to_string_pretty(&pokemon, PrettyConfig::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }

    std::fs::create_dir_all("generated/items").unwrap();

    for item in generated.items.items.into_iter() {
        std::fs::write(
            format!("generated/items/{}.ron", item.id),
            ron::ser::to_string_pretty(&item, Default::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    }

    std::fs::create_dir_all("generated/client/items").unwrap();

    for (id, item) in generated.items.textures {
        std::fs::write(format!("generated/client/items/{}.png", id), &item).unwrap();
    }
}
