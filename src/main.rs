// src/main.rs

// region: --- modules
use response_engine::speak_ollama;
use std::error::Error;
// endregion: --- modules

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt_final = format!(
        "{}",
        "tell me something interesting use of pink floyd in three sentences"
    );
    let _ = speak_ollama(prompt_final).await;
    // speak_clipboard().await;
    Ok(())
}
