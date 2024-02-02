// src/utils/clipboard.rs

// region: --- modules
use crate::speak_text;
use std::error::Error;
use std::process::Command;
// endregion: --- modules

pub async fn speak_clipboard() -> Result<(), Box<dyn Error>> {
    let clipboard_result = get_clipboard();
    match clipboard_result {
        Ok(clipboard_content) => {
            // Assuming speak_text() is async and returns a Result<(), Box<dyn Error>>
            speak_text(&clipboard_content).await?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error getting clipboard content: {}", err);
            Err(err)
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
