// src/utils/audio.rs

// region: --- modules
use crate::{azure_response_to_audio, get_azure_response, AudioPlaybackManager};
use actix_web::web;
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::io::{self, Cursor};
use std::sync::Mutex;
// endregion: --- modules

// Main speak_text function (now asynchronous) using the simplified logic
pub async fn speak_text(text: &str) -> Result<(), Box<dyn Error>> {
    let azure_response = get_azure_response(text).await?;
    let audio_data = azure_response_to_audio(azure_response).await?;
    // play_audio_data(audio_data).await?;
    Ok(())
}

// Simplified function to play audio directly from memory
pub async fn play_audio_data(
    data: web::Data<Mutex<AudioPlaybackManager>>,
    audio_data: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let mut manager = data.lock().unwrap();
    manager.play_audio(audio_data)?;
    Ok(())
}
