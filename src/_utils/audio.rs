// src/utils/audio.rs

// region: --- modules
use std::error::Error;
use tokio::sync::mpsc;

use crate::{azure_response_to_audio, get_azure_response, PlaybackCommand};
// endregion: --- modules

pub async fn speak_text(
    text: &str,
    tx: mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let azure_response = get_azure_response(text).await?;
    let audio_data = azure_response_to_audio(azure_response).await?;
    // Instead of sending audio data directly, wrap it in a PlaybackCommand::Play
    tx.send(PlaybackCommand::Play(audio_data))
        .await
        .map_err(|e| e.into()) // Convert send error
}
