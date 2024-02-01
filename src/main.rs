use chrono::prelude::*;
use dotenv::dotenv;
use response_engine::{get_azure_response, listen_to_audio_stream, ollama_generate_api};
use std::env;
use std::error::Error;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt_final = format!(
        "{}",
        "tell me something interesting about rust land in three sentences"
    );

    // Create a channel for sending generated text
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a separate task to generate sentences concurrently
    tokio::spawn(async move {
        ollama_generate_api(prompt_final, tx)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to generate sentences: {}", e);
            });
    });

    while let Some(sentence) = rx.recv().await {
        println!("Sentence Received: {}", sentence);
        let tts_response = get_azure_response(&sentence).await?;
        listen_to_audio_stream(tts_response).await?;
    }
    Ok(())
}
