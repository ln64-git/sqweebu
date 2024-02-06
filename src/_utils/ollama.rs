// src/api/ollama.rs

// region: --- Modules
use crate::PlaybackCommand;
use crate::_utils::azure::speak_text;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
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

pub async fn speak_ollama(
    prompt: String,
    control_tx: Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let (sentence_tx, mut sentence_rx) = mpsc::channel::<String>(32);
    tokio::spawn(async move {
        if let Err(e) = ollama_generate_api(prompt.clone(), sentence_tx).await {
            eprintln!("Failed to generate sentences: {}", e);
        }
    });
    while let Some(sentence) = sentence_rx.recv().await {
        // send a command to play the audio.
        if let Err(e) = speak_text(&sentence, control_tx.clone()).await {
            eprintln!("Error processing sentence to audio: {}", e);
        }
    }
    Ok(())
}

pub async fn ollama_generate_api(
    final_prompt: String,
    inner_tx: mpsc::Sender<String>,
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
                        inner_tx.send(accumulated_response.clone()).await?;
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
        inner_tx.send(accumulated_response).await?;
    }
    Ok(())
}
