use std::error::Error;
use std::process::Command;

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
