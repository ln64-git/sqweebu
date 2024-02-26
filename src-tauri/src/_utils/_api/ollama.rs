// src/_utils/ollama.rs

// region: --- Modules

use crate::AppState;
use crate::PlaybackCommand;
use reqwest;
use sentence::SentenceTokenizer;
use sentence::Token;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

use super::azure::speak_text;

// endregion: --- Modules

// region: --- Structs

#[derive(Deserialize, Serialize)] // Make sure to derive Deserialize
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaFragment {
    response: String,
    done: bool,
}

// endregion: --- Structs

pub async fn speak_ollama(
    prompt: String,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let mut index = 1;
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);

    tokio::spawn(async move {
        match ollama_generate_api(prompt.clone(), sentence_send).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to generate sentences: {}", e),
        }
    });

    while let Some(sentence) = sentence_recv.recv().await {
        speak_text(&sentence, &playback_send).await?;
    }

    Ok(())
}

// region: --- Ollama API

pub async fn ollama_generate_api(
    final_prompt: String,
    sentence_send: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request_body = GenerateRequest {
        model: "dolphin-mixtral".to_string(),
        // model: "llama2-uncensored".to_string(),
        prompt: final_prompt,
        stream: true,
    };

    let mut response_stream = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?
        .bytes_stream();

    let mut stream_ended = false;
    let mut sentence = String::new();

    while let Some(chunk) = response_stream.next().await {
        let chunk = chunk?;
        let chunk_text = String::from_utf8_lossy(&chunk);

        for line in chunk_text.split('\n').filter(|s| !s.is_empty()) {
            match serde_json::from_str::<OllamaFragment>(line) {
                Ok(fragment) => {
                    sentence.push_str(&fragment.response);
                    if detect_punctuation(fragment).await {
                        let final_sentence = parse_sentence(&sentence).await;
                        sentence_send.send(final_sentence).await; // await here
                        sentence.clear();
                    }
                }
                Err(e) => {
                    eprintln!("JSON parsing error: {}", e);
                }
            }
        }
    }
    stream_ended = true;
    Ok(())
}

async fn parse_sentence(sentence: &String) -> String {
    let cleaned_sentence = if sentence.starts_with('\n') {
        sentence.chars().skip(1).collect()
    } else {
        sentence.clone()
    };
    cleaned_sentence
}

async fn detect_punctuation(fragment: OllamaFragment) -> bool {
    let text_fragment = fragment.response;
    let tokenizer = SentenceTokenizer::new();
    let tokens = tokenizer.tokenize(text_fragment.as_str());
    for token in tokens {
        match token {
            Token::Punctuation(punctuation) => return true,
            _ => {}
        }
    }
    return false;
}

// endregion: --- Ollama API
