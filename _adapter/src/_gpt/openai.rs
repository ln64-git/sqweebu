// src/_utils/chatgpt.rs

// region: --- Modules

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use tokio::sync::mpsc;

// endregion: --- Modules

// region: --- Structs

#[derive(Deserialize, Serialize)]
struct GenerateRequest {
    prompt: String,
    max_tokens: usize,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<ChatGPTChoice>,
}

#[derive(Deserialize)]
struct ChatGPTChoice {
    text: String,
}

// endregion: --- Structs

// region: --- ChatGPT API

pub async fn chatgpt_generate_api(
    final_prompt: String,
    sentence_send: mpsc::Sender<String>,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request_body = GenerateRequest {
        prompt: final_prompt,
        max_tokens: 150,  // Adjust as needed
        temperature: 0.7, // Adjust as needed
    };

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer YOUR_API_KEY") // Replace YOUR_API_KEY with your OpenAI API key
        .json(&request_body)
        .send()
        .await?;

    let response_body = response.text().await?;
    let parsed_response: ChatGPTResponse = serde_json::from_str(&response_body)?;

    for choice in parsed_response.choices {
        let final_sentence = parse_sentence(&choice.text).await;
        let _ = sentence_send.send(final_sentence).await; // await here
    }

    Ok(())
}

async fn parse_sentence(sentence: &str) -> String {
    let cleaned_sentence = sentence.trim().to_string();
    cleaned_sentence
}
// endregion: --- Ollama API
