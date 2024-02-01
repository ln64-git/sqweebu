use chrono::prelude::*;
use dotenv::dotenv;
use response_engine::{generate_text_api, get_azure_response, listen_to_audio_stream};
use std::env;
use std::error::Error;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let start_time = Utc::now();

    let model = "llama2-uncensored";
    let subscription_key = env::var("API_KEY").unwrap();
    let region = "eastus";
    let voice_gender = "Female";
    let voice_name = "en-US-JennyNeural";
    let output_format = "audio-48khz-192kbitrate-mono-mp3";

    let prompt_final = format!("{}{}", "In three sentences, Explain this...", "Rust fold and unfold");

    // Create a channel for sending generated text
    let (tx, mut rx) = mpsc::channel(32);

    // Run generate_text in the same async context
    generate_text_api(model, prompt_final, tx).await?;
    let after_generate_text_api = Utc::now();
    println!(
        "Time for generate_text_api: {} seconds",
        after_generate_text_api
            .signed_duration_since(start_time)
            .num_seconds()
    );

    // Process the sentences received from the channel
    while let Some(sentence) = rx.recv().await {
        let sentence_received_time = Utc::now();
        println!(
            "Received sentence: {}. Time since start: {} seconds",
            sentence,
            sentence_received_time
                .signed_duration_since(start_time)
                .num_seconds()
        );

        let tts_response = get_azure_response(
            &subscription_key,
            &region,
            &sentence,
            &voice_gender,
            &voice_name,
            &output_format,
        )
        .await?;
        listen_to_audio_stream(tts_response).await?;
    }
    let time_till_complete = Utc::now();
    println!(
        "Time till completeion: {} seconds",
        time_till_complete
            .signed_duration_since(start_time)
            .num_seconds()
    );
    Ok(())
}
