use std::{path::PathBuf, process::Stdio};

use tokio::process::Command;

pub async fn get_cry(folder: &PathBuf, pokemon: &str) {
    let response = reqwest::get(&format!("https://raw.githubusercontent.com/pret/pokefirered/master/sound/direct_sound_samples/cry_{}.aif", pokemon)).await.unwrap_or_else(|err| panic!("Could not get web response for cry of {}. Error: {}", pokemon, err));
    let bytes = response.bytes().await.unwrap_or_else(|err| panic!("Could not get cry bytes for {} with error {}", pokemon, err));
    let folder = folder.to_string_lossy();
    let temp_cry = format!("{}/temp_cry.aif", folder);
    let cry = format!("{}/cry.ogg", folder);
    tokio::fs::write(&temp_cry, &bytes).await.unwrap_or_else(|err| panic!("Could not write temporary cry file at {:?} for {} with error {}", pokemon, &temp_cry, err));
    let mut command = Command::new(super::FFMPEG_PATH);
    command.stdout(Stdio::null()).kill_on_drop(true);
    command.arg("-i").arg(&temp_cry).arg(&cry);
    command.output().await.unwrap_or_else(|err| panic!("Could not execute ffmpeg for {} with error {}", pokemon, err));
    tokio::fs::remove_file(&temp_cry).await.unwrap_or_else(|err| panic!("Could not remove temporary cry file for {} with error {}", pokemon, err));
}