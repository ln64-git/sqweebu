use crate::UserSettings;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tauri::api::path::config_dir;

// Function to get the path to the user settings file
pub fn get_user_settings_path() -> Option<PathBuf> {
    if let Some(mut config_dir) = config_dir() {
        config_dir.push("sqweebu");
        fs::create_dir_all(&config_dir).ok()?;
        config_dir.push("user_settings.json");
        Some(config_dir)
    } else {
        None
    }
}

// Example function to save user settings to a file
pub fn save_user_settings(user_settings: &UserSettings) -> Result<(), Box<dyn Error>> {
    if let Some(path) = get_user_settings_path() {
        let serialized = serde_json::to_string(user_settings)?;
        fs::write(path, serialized)?;
        Ok(())
    } else {
        Err("Unable to get configuration directory".into())
    }
}

// Example function to load user settings from a file
pub fn load_user_settings() -> Result<UserSettings, Box<dyn Error>> {
    if let Some(path) = get_user_settings_path() {
        let content = fs::read_to_string(path)?;
        let user_settings: UserSettings = serde_json::from_str(&content)?;
        Ok(user_settings)
    } else {
        Err("Unable to get configuration directory".into())
    }
}
