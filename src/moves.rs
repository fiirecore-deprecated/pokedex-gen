use anyhow::Result;
use pokerust::{FromId, Id};
use std::path::Path;
use std::sync::Arc;

use pokedex::{
    moves::{
        target::MoveTarget,
        usage::{DamageKind, MoveUseType},
        Move, MoveCategory,
    },
    pokemon::{
        stat::{BattleStatType, StatType},
    },
    status::{Status, StatusRange},
};

const MOVES_SIZE: i16 = 559;

const MOVES_PATH: &str = "pokedex/moves/";

pub async fn add_moves(client: Arc<reqwest::Client>) -> Result<()> {
    let path = Path::new(MOVES_PATH);
    if !path.exists() {
        tokio::fs::create_dir(&path).await?;
    }

    for index in 1..MOVES_SIZE {
        let client = client.clone();
        tokio::task::spawn(async move {
            get_move(index, client.as_ref(), &path).await;
        });
    }

    Ok(())
}

async fn get_move(index: i16, client: &reqwest::Client, path: &Path) {
    let mut move_ = pokerust::Move::from_id(client, index)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Could not get move from id {} with error {}", index, err);
            panic!()
        });

    let name = move_.names.remove(7).name;

    let id = move_
        .name
        .parse()
        .expect("Could not parse move name into ASCII string!");

    crate::capitalize_first(&mut move_.type_.name);
    crate::capitalize_first(&mut move_.damage_class.name);

    println!("Creating move entry for: {}", name);

    tokio::fs::write(
        path.join(format!("{}.ron", name)),
        ron::ser::to_string_pretty(
            &Move {
                id,
                pp: move_
                    .pp
                    .unwrap_or_else(|| panic!("Could not get PP for pokemon move {}", name)),
                name,
                category: category_from_id(move_.damage_class.id()),
                pokemon_type: crate::type_from_id(move_.type_.id()),
                accuracy: move_.accuracy,
                target: target_from_id(move_.target.id()),
                priority: move_.priority,
                contact: false,
                crit_rate: move_
                    .meta
                    .as_ref()
                    .map(|meta| meta.crit_rate)
                    .unwrap_or_default(),
                field_id: None,
                usage: get_move_usage(&move_),
            },
            ron::ser::PrettyConfig::default(),
        )
        .unwrap_or_else(|err| {
            eprintln!("Could not serialize move {} with error {}", move_.name, err);
            panic!()
        })
        .as_bytes(),
    )
    .await
    .unwrap_or_else(|err| {
        eprintln!("Could not write move {} to file with error {}", move_.name, err);
        panic!()
    });
}

fn category_from_id(id: i16) -> MoveCategory {
    match id {
        2 => MoveCategory::Physical,
        3 => MoveCategory::Special,
        1 => MoveCategory::Status,
        _ => panic!("Could not get move category from id \"{}\"", id),
    }
}

fn get_move_usage(move_: &pokerust::Move) -> Vec<MoveUseType> {
    let mut usages = Vec::with_capacity(1);

    if let Some(power) = move_.power {
        usages.push(MoveUseType::Damage(DamageKind::Power(power)))
    }

    // metadata

    if let Some(metadata) = move_.meta.as_ref() {

        // flinch check

        if metadata.flinch_chance != 0 {
            let chance = metadata.flinch_chance as f32 / 100.0;
            let flinch = vec![MoveUseType::Flinch];
            usages.push(if metadata.flinch_chance == 100 {
                MoveUseType::Flinch
            } else {
                MoveUseType::Chance(flinch, chance)
            });
        }

        // drain check

        if metadata.drain != 0 {
            if let Some(MoveUseType::Damage(kind)) = usages.get(0) {
                usages[0] = MoveUseType::Drain(*kind, metadata.drain as f32 / 100.0);
            }
        }

        // status effect check

        if !matches!(metadata.ailment.id(), -1 | 0) {

            if let Some((status, range)) = match metadata.ailment.id() {
                1 => Some((Status::Paralysis, status_range(metadata.min_turns, metadata.max_turns))),
                2 => Some((Status::Sleep, status_range(metadata.min_turns, metadata.max_turns))),
                3 => Some((Status::Freeze, status_range(metadata.min_turns, metadata.max_turns))),
                4 => Some((Status::Burn, status_range(metadata.min_turns, metadata.max_turns))),
                5 => Some((Status::Poison, status_range(metadata.min_turns, metadata.max_turns))),
                _ => None,
            } {

                let chance = metadata.ailment_chance as f32 / 100.0;
                usages.push(MoveUseType::Status(status, range, chance));

            }

        }

        // stat stage check

        if !move_.stat_changes.is_empty() {

            let stat_chance = metadata.stat_chance;

            let stat_changes = move_
            .stat_changes
            .iter()
            .map(|stat| MoveUseType::StatStage(get_stat_type(stat.stat.id(), &move_.name), stat.change));
    
            if matches!(stat_chance, 0 | 100) {
                usages.extend(stat_changes);
            } else {
                let chance = stat_chance as f32 / 100.0;
                usages.push(MoveUseType::Chance(stat_changes.collect(), chance));
            }
        }
    }

    if usages.is_empty() {
        usages.push(MoveUseType::Todo)
    }

    usages
}

fn status_range(min_turns: Option<u8>, max_turns: Option<u8>) -> StatusRange {
    match min_turns.zip(max_turns) {
        Some((min, max)) => StatusRange::Temporary(min, max),
        None => StatusRange::Permanent,
    }
}

fn get_stat_type(id: i16, name: &str) -> BattleStatType {
    match id {
        1 => BattleStatType::Basic(StatType::Health),
        2 => BattleStatType::Basic(StatType::Attack),
        3 => BattleStatType::Basic(StatType::Defense),
        4 => BattleStatType::Basic(StatType::SpAttack),
        5 => BattleStatType::Basic(StatType::SpDefense),
        6 => BattleStatType::Basic(StatType::Speed),
        7 => BattleStatType::Accuracy,
        8 => BattleStatType::Evasion,
        id => {
            eprintln!("Move {} has unknown battle stat type id {}", name, id);
            panic!()
        },
    }
}

fn target_from_id(target: i16) -> MoveTarget {
    match target {
        3 => MoveTarget::Ally,
        5 => MoveTarget::UserOrAlly,
        13 => MoveTarget::UserAndAllies,
        9 => MoveTarget::AllOtherPokemon,
        10 => MoveTarget::Any,
        11 => MoveTarget::AllOpponents,
        15 => MoveTarget::Allies,
        // 13 => MoveTarget::UserOrAllies,
        7 => MoveTarget::User,
        14 => MoveTarget::AllPokemon,
        _ => MoveTarget::Todo,
    }
}
