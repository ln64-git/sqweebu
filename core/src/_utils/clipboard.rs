// src/_utils/clipboard.rs

// region: --- modules
use crate::PlaybackCommand;
use crate::_utils::azure::speak_text;
use std::error::Error;
use std::process::Command;
use tokio::sync::mpsc::Sender;
// endregion: --- modules

pub async fn speak_clipboard(playback_tx: Sender<PlaybackCommand>) -> Result<(), Box<dyn Error>> {
    let clipboard_result = get_clipboard();
    match clipboard_result {
        Ok(clipboard_content) => {
            speak_text(&clipboard_content, playback_tx).await?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error getting clipboard content: {}", err);
            Err(err.into())
        }
    }
}

pub fn get_clipboard() -> Result<String, Box<dyn Error>> {
    let output = Command::new("wl-paste").output()?;
    if !output.status.success() {
        return Err(format!(
            "Command failed with status: {:?}, stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
