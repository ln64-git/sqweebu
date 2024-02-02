// region: --- modules
use chrono::prelude::*;
use dotenv::dotenv;
use response_engine::{
    get_azure_response, get_clipboard, listen_to_audio_stream, ollama_generate_api,
    speak_clipboard, speak_ollama, speak_text,
};
use std::env;
use std::error::Error;
use tokio::sync::mpsc;
// endregion: --- modules

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt_final = format!(
        "{}",
        "tell me something interesting about deep sea life in three sentences"
    );
    // speak_ollama(prompt_final).await;
    speak_clipboard().await;
    Ok(())
}
