// src/api/ollama.rs

// region: --- Modules
use crate::{get_azure_response, play_audio_data, speak_text};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
// endregion: --- Modules

#[derive(Deserialize, Serialize)] // Make sure to derive Deserialize
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct PartialGenerateResponse {
    response: String,
}

pub async fn speak_ollama(prompt: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        ollama_generate_api(prompt.clone(), tx)
            .await
            .unwrap_or_else(|e| eprintln!("Failed to generate sentences: {}", e));
    });
    let mut combined_audio_data = Vec::new();
    while let Some(sentence) = rx.recv().await {
        // Assuming speak_text now returns Vec<u8> of audio for the sentence
        let audio_data = speak_text(&sentence).await?;
        println!("audio_data.len() = {}", audio_data.len());
        // Here you would append or mix the audio data as needed
        combined_audio_data.extend(audio_data);
    }
    Ok(combined_audio_data)
}

pub async fn ollama_generate_api(
    final_prompt: String,
    tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request_body = GenerateRequest {
        model: "llama2-uncensored".to_string(),
        prompt: final_prompt,
        stream: true,
    };

    let mut response_stream = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?
        .bytes_stream();

    let mut accumulated_response = String::new();

    while let Some(chunk) = response_stream.next().await {
        let chunk = chunk?;
        let chunk_text = String::from_utf8_lossy(&chunk);

        for line in chunk_text.split('\n').filter(|s| !s.is_empty()) {
            match serde_json::from_str::<PartialGenerateResponse>(line) {
                Ok(partial_response) => {
                    accumulated_response.push_str(&partial_response.response);
                    if accumulated_response.ends_with(['.', '?', '!']) {
                        tx.send(accumulated_response.clone()).await?;
                        accumulated_response.clear();
                    }
                }
                Err(e) => {
                    eprintln!("JSON parsing error: {}", e);
                }
            }
        }
    }
    if !accumulated_response.is_empty() {
        tx.send(accumulated_response).await?;
    }
    Ok(())
}
