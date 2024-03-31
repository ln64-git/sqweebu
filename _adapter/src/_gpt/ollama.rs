// src/_utils/ollama.rs

// region: --- Modules

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

// endregion: --- Modules

// region: --- Structs

#[derive(Deserialize, Serialize)] // Make sure to derive Deserialize
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct OllamaFragment {
    response: String,
    done: bool,
}

// endregion: --- Structs

// region: --- Ollama API

pub async fn ollama_generate_api(
    prompt: String,
    sentence_send: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request_body = GenerateRequest {
        model: "llama2-uncensored".to_string(), // Adjusted for clarity.
        prompt,
        stream: true,
    };
    let mut response_stream = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?
        .bytes_stream();

    let mut sentence = String::new();
    while let Some(chunk) = response_stream.next().await {
        let chunk = chunk?;
        let chunk_text = String::from_utf8_lossy(&chunk);

        for line in chunk_text.split('\n').filter(|s| !s.is_empty()) {
            match serde_json::from_str::<OllamaFragment>(line) {
                Ok(fragment) => {
                    sentence.push_str(&fragment.response);
                    if detect_sentence_end(&fragment).await {
                        if !sentence.trim().is_empty() {
                            let _ = sentence_send.send(sentence.clone()).await;
                        } else {
                            eprintln!("Skipping empty or whitespace-only sentence.");
                        }
                        sentence.clear();
                    }
                }
                Err(e) => eprintln!("JSON parsing error: {}", e),
            }
        }
    }
    Ok(())
}

async fn detect_sentence_end(fragment: &OllamaFragment) -> bool {
    // Check if the fragment is marked as done by the API.
    if fragment.done {
        return true;
    }

    // Detect sentence-ending punctuation.
    let text_fragment = &fragment.response;
    let ending_chars = text_fragment
        .trim_end_matches(char::is_alphanumeric)
        .chars()
        .rev();

    let mut found_ending_punctuation = false;
    for c in ending_chars {
        match c {
            '.' | '!' | '?' | ',' => {
                found_ending_punctuation = true;
            }
            // Assuming space, quotation marks, or other punctuation might follow the sentence-ending punctuation.
            ' ' | '"' | '\'' | ')' | ']' | '}' => continue,
            // If we encounter any other character before finding a sentence-ending punctuation, stop the check.
            _ => break,
        }
    }

    found_ending_punctuation
}

// endregion: --- Ollama API
