// src/utils/clipboard.rs

// region: --- modules
use crate::speak_text;
use std::error::Error;
use std::process::Command;
// endregion: --- modules

pub async fn speak_clipboard() {
    let clipboard_result = get_clipboard();
    match clipboard_result {
        Ok(clipboard) => {
            let clipboard_str = clipboard.as_str();
            let _ = speak_text(clipboard_str).await;
        }
        Err(err) => {
            eprintln!("Error getting clipboard content: {}", err);
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
