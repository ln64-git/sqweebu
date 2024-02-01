use chrono::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct PartialGenerateResponse {
    response: String,
    done: Option<bool>,
}

pub async fn generate_text_api(
    model: &str,
    final_prompt: String,
    tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request_body = GenerateRequest {
        model: model.to_string(),
        prompt: final_prompt,
        stream: true, // Enable streaming
    };

    let start_time = Utc::now();

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
                        let sentence_found = Utc::now();
                        println!(
                            "Sentence found after {} seconds: {}",
                            sentence_found
                                .signed_duration_since(start_time)
                                .num_seconds(),
                            accumulated_response,
                        );
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

    // Send any remaining text after the stream ends
    if !accumulated_response.is_empty() {
        tx.send(accumulated_response).await?;
    }

    Ok(())
}
