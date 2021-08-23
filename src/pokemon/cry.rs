use std::{path::PathBuf, process::Stdio};

use anyhow::Result;
use tokio::process::Command;

pub async fn get_cry(folder: &PathBuf, pokemon: &str) -> Result<()> {
    let response = reqwest::get(&format!("https://raw.githubusercontent.com/pret/pokefirered/master/sound/direct_sound_samples/cry_{}.aif", pokemon)).await?;
    let bytes = response.bytes().await?;
    let folder = format!("{}{}", super::CLIENT_POKEMON_PATH, folder.to_string_lossy());
    let temp_cry = format!("{}/temp_cry.aif", folder);
    let cry = folder + "/cry.ogg";
    tokio::fs::write(&temp_cry, &bytes).await?;
    let mut command = Command::new(super::FFMPEG_PATH);
    command.stdout(Stdio::null()).kill_on_drop(true);
    command.arg("-i").arg(&temp_cry).arg(&cry);
    command.output().await?;
    tokio::fs::remove_file(&temp_cry).await?;
    Ok(())
}