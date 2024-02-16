// src/_utils/ollama.rs

// region: --- Modules
use crate::PlaybackCommand;
use crate::_utils::azure::speak_text;
use reqwest;
use sentence::SentenceTokenizer;
use sentence::Token;
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
struct OllamaFragment {
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
        println!("sentence retreived: {:#?}", sentence);

        sentence_array.push(sentence);
    }

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
    sentence_send: mpsc::Sender<String>,
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

    let mut stream_ended = false; // Flag to track if response stream has ended
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
    // Set stream_ended to true when the response stream ends
    stream_ended = true;
    // Send completion signal
    let _ = ollama_complete_send.send(true).await; // await here
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
