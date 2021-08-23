use anyhow::Result;
use log::{error, info, warn};
use pokerust::Id;
use std::path::Path;
use std::sync::Arc;

use battle::{
    moves::usage::{DamageKind, MoveAction, MoveExecution, MoveUsage},
    pokedex::{
        ailment::{Ailment, AilmentLength},
        moves::{Move, MoveCategory, MoveTarget},
        pokemon::stat::StatType,
    },
    pokemon::battle::stat::BattleStatType,
};

const MOVES_SIZE: i16 = 559;

const MOVES_PATH: &str = "pokedex/moves/";
const BATTLE_PATH: &str = "pokedex/battle/";
const SCRIPT_PATH: &str = "pokedex/battle/scripts";

pub async fn add_moves(client: Arc<pokerust::Client>) -> Result<()> {
    let path = Path::new(MOVES_PATH);
    if !path.exists() {
        tokio::fs::create_dir_all(&path).await?;
    }

    let path = Path::new(SCRIPT_PATH);
    if !path.exists() {
        tokio::fs::create_dir_all(&path).await?;
    }

    for index in 1..MOVES_SIZE {
        let client = client.clone();
        tokio::task::spawn(async move {
            get_move(index, client.as_ref()).await;
        });
    }

    Ok(())
}

async fn get_move(index: i16, client: &pokerust::Client) {
    let mut move_ = client
        .get::<pokerust::Move, i16>(index)
        .await
        .unwrap_or_else(|err| {
            error!("Could not get move from id {} with error {}", index, err);
            panic!()
        });

    let name = move_.names.remove(7).name;

    let id = move_
        .name
        .parse()
        .expect("Could not parse move name into ASCII string!");

    crate::capitalize_first(&mut move_.type_.name);
    crate::capitalize_first(&mut move_.damage_class.name);

    info!("Creating move entry for: {}", name);

    tokio::fs::write(
        format!("{}{}.ron", BATTLE_PATH, name),
        ron::ser::to_string_pretty(
            &(
                id,
                MoveUsage {
                    execute: get_move_usage(&move_),
                    contact: false,
                    crit_rate: move_
                        .meta
                        .as_ref()
                        .map(|meta| meta.crit_rate)
                        .unwrap_or_default(),
                }
            ),
            Default::default(),
        )
        .unwrap_or_else(|err| {
            error!("Could not serialize move usage for {} with error {}", move_.name, err);
            panic!()
        })
    )
    .await
    .unwrap_or_else(|err| {
        error!(
            "Could not write move usage for {} to file with error {}",
            move_.name, err
        );
        panic!()
    });

    tokio::fs::write(
        format!("{}{}.ron", MOVES_PATH, name),
        ron::ser::to_string_pretty(
            &Move {
                id,
                pp: move_
                    .pp
                    .unwrap_or_else(|| panic!("Could not get PP for pokemon move {}", name)),
                name,
                category: category_from_id(move_.damage_class.id()),
                pokemon_type: crate::type_from_id(move_.type_.id()),
                power: move_.power,
                accuracy: move_.accuracy,
                priority: move_.priority,
                target: target_from_id(move_.target.id()),
                world: is_world_move(&move_),
            },
            Default::default(),
        )
        .unwrap_or_else(|err| {
            error!("Could not serialize move {} with error {}", move_.name, err);
            panic!()
        })
    )
    .await
    .unwrap_or_else(|err| {
        error!(
            "Could not write move {} to file with error {}",
            move_.name, err
        );
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

fn get_move_usage(move_: &pokerust::Move) -> MoveExecution {
    match move_.name.as_str() {
        "false-swipe" => MoveExecution::Script(move_.name.parse().unwrap()),
        _ => {
            let actions = get_move_actions(move_);
            match actions.is_empty() {
                true => MoveExecution::None,
                false => MoveExecution::Actions(actions),
            }
        }
    }
}

fn get_move_actions(move_: &pokerust::Move) -> Vec<MoveAction> {
    let mut usages = Vec::with_capacity(1);

    if let Some(power) = move_.power {
        usages.push(MoveAction::Damage(DamageKind::Power(power)))
    }

    // metadata

    if let Some(metadata) = move_.meta.as_ref() {
        // flinch check

        if metadata.flinch_chance != 0 {
            let flinch = vec![MoveAction::Flinch];
            usages.push(if metadata.flinch_chance == 100 {
                MoveAction::Flinch
            } else {
                MoveAction::Chance(flinch, metadata.flinch_chance)
            });
        }

        // drain check

        if metadata.drain != 0 {
            if let Some(MoveAction::Damage(kind)) = usages.get(0) {
                usages[0] = MoveAction::Drain(*kind, metadata.drain);
            }
        }

        // status effect check

        if !matches!(metadata.ailment.id(), -1 | 0) {
            let range = status_range(metadata.min_turns, metadata.max_turns);

            if let Some(ailment) = match metadata.ailment.id() {
                1 => Some(Ailment::Paralysis),
                2 => Some(Ailment::Sleep),
                3 => Some(Ailment::Freeze),
                4 => Some(Ailment::Burn),
                5 => Some(Ailment::Poison),
                id => {
                    warn!("Could not get ailment #{}", id);
                    None
                }
            } {
                usages.push(MoveAction::Ailment(ailment, range, metadata.ailment_chance));
            }
        }

        // stat stage check

        if !move_.stat_changes.is_empty() {
            let stat_changes = move_.stat_changes.iter().map(|stat| {
                MoveAction::Stat(get_stat_type(stat.stat.id(), &move_.name), stat.change)
            });

            if matches!(metadata.stat_chance, 0 | 100) {
                usages.extend(stat_changes);
            } else {
                usages.push(MoveAction::Chance(
                    stat_changes.collect(),
                    metadata.stat_chance,
                ));
            }
        }
    }

    // if usages.is_empty() {
    //     usages.push(MoveAction::Todo)
    // }

    usages
}

/// 15 = Cut, 19 = Fly, 57 = Surf, 70 = Strength, 127 = Waterfall, 249 = Rock Smash
fn is_world_move(move_: &pokerust::Move) -> bool {
    match move_.id {
        15 | 19 | 57 | 70 | 127 | 249 => true,
        _ => false,
    }
}

fn status_range(min_turns: Option<u8>, max_turns: Option<u8>) -> AilmentLength {
    match min_turns.zip(max_turns) {
        Some((min, max)) => AilmentLength::Temporary(min, max),
        None => AilmentLength::Permanent,
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
            error!("Move {} has unknown battle stat type id {}", name, id);
            panic!()
        }
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
        _ => MoveTarget::None,
    }
}
