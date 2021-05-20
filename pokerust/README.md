# Pokérust

[![Crates.io](https://img.shields.io/crates/v/pokerust?style=flat-square)](https://crates.io/crates/pokerust)
[![Crates.io](https://img.shields.io/crates/d/pokerust?style=flat-square)](https://crates.io/crates/pokerust)
[![Crates.io](https://img.shields.io/crates/l/pokerust?style=flat-square)](LICENSE)

Wrapper library for <https://pokeapi.co/> v2 with caching support.

## Documentation

Documentation for the crate can be found on
[docs.rs](https://docs.rs/pokerust) (WIP). For documentation of the API, see
<https://pokeapi.co/docs/v2.html>.

## Basic Usage

Get an object from an API by id

```rust
use pokerust::{Berry, FromId};

fn main() {
    let berry = Berry::from_id(1).unwrap();
}
```

or by name

```rust
use pokerust::{Berry, FromName};

fn main() {
    let berry = Berry::from_name("cheri").unwrap();
}
```

API responses are automatically cached.

You can also fetch the resource lists:

```rust
let items = Item::list(5, 20)?;  // ?offset=5&limit=20

// get the lists referenced in the next and previous fields
items.previous_list()?;
items.next_list()?;

// you can also just get the full list
let all_items = Item::full_list()?;
```

To get resources pointed to by `(Named)APIResource`, use `get()`:

```rust
let berry = Berry::from_name("cheri")?;
let berry_item = berry.item.get()?; // berry_item is an Item
```

This can be chained:

```rust
let marill = PokemonSpecies::from_name("marill")?;
let sea_incense = marill.evolution_chain.get()?.baby_trigger_item.unwrap().get()?;
```

The location of the pokeapi used can be changed by setting the
POKERUST_ENDPOINT environment variable. Defaults to the public instance at
<https://pokeapi.co/api/v2/>. Please consult the pokeapi documentation and read
the fair use policy before using the public API instance.

## License

This software is licensed under the BSD 3-Clause "New" or "Revised" License.
