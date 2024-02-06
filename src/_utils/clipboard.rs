// src/utils/clipboard.rs

// region: --- modules

use crate::{speak_text, PlaybackCommand};
use std::error::Error;
use std::process::Command;
use tokio::sync::mpsc::Sender;
// endregion: --- modules

pub async fn speak_clipboard(control_tx: Sender<PlaybackCommand>) -> Result<(), Box<dyn Error>> {
    let clipboard_result = get_clipboard();
    match clipboard_result {
        Ok(clipboard_content) => {
            // Pass the sender to speak_text, adjusted for PlaybackCommand.
            speak_text(&clipboard_content, control_tx).await?;
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
