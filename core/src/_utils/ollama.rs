// src/_utils/ollama.rs

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
    done: bool,
}

pub async fn speak_ollama(
    prompt: String,
    playback_send: Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);
    let (ollama_complete_send, mut ollama_complete_recv) = mpsc::channel::<bool>(32);

    // Spawn async task to generate sentences
    tokio::spawn(async move {
        if let Err(e) =
            ollama_generate_api(prompt.clone(), sentence_send, ollama_complete_send).await
        {
            eprintln!("Failed to generate sentences: {}", e);
        }
    });

    let mut sentence_array: Vec<String> = Vec::new();

    // Receive sentences and populate the sentence_array
    while let Some(sentence) = sentence_recv.recv().await {
        sentence_array.push(sentence);
    }

    println!("sentence_queue: {:#?}", sentence_array);

    // Receive completion signal from ollama_generate_api
    if let Some(_) = ollama_complete_recv.recv().await {
        // Process completion
        Ok(())
    } else {
        // Handle error if completion signal is not received
        Err("Completion signal not received".into())
    }
}

pub async fn ollama_generate_api(
    final_prompt: String,
    inner_send: mpsc::Sender<String>,
    ollama_complete_send: mpsc::Sender<bool>,
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
    let mut stream_ended = false; // Flag to track if response stream has ended

    while let Some(chunk) = response_stream.next().await {
        let chunk = chunk?;
        let chunk_text = String::from_utf8_lossy(&chunk);

        for line in chunk_text.split('\n').filter(|s| !s.is_empty()) {
            match serde_json::from_str::<PartialGenerateResponse>(line) {
                Ok(partial_response) => {
                    accumulated_response.push_str(&partial_response.response);
                    if partial_response.done {
                        stream_ended = true;
                    }
                    if accumulated_response.ends_with(['.', '?', '!']) {
                        inner_send.send(accumulated_response.clone()).await?;
                        accumulated_response.clear();
                    }
                }
                Err(e) => {
                    eprintln!("JSON parsing error: {}", e);
                }
            }
        }

        // Check if the stream has ended
        if stream_ended {
            break; // Exit the loop as the stream has ended
        }
    }

    // Send the remaining accumulated response
    if !accumulated_response.is_empty() {
        inner_send.send(accumulated_response).await?;
    }

    // Send completion signal if the stream has ended
    if stream_ended {
        let _ = ollama_complete_send.send(true).await;
    }

    Ok(())
}

// Collect sentences in an array here
// Keep track of sink completion state
// if sink_completed == true
// let _ = speak_text(&sentence, playback_send.clone()).await

// while let Some(sentence) = sentence_recv.recv().await {
//     println!("---------------------------------------");
//     println!("SPEAK_OLLAMA - Sentence Retrieved: ");
//     println!("{}", sentence);
//     println!("---------------------------------------");
//     // send a command to play the audio.
//     if let Err(e) = speak_text(&sentence, playback_send.clone()).await {
//         eprintln!("Error processing sentence to audio: {}", e);
//     }
// }
