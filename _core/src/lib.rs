// Export items from playback.rs
pub mod playback;
use std::error::Error;

use _adapter::{azure::get_azure_audio_response, ollama::ollama_generate_api};
use _interface::PlaybackCommand;
pub use playback::*;
use tokio::sync::mpsc;

pub async fn speak_text(
    text: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_azure_audio_response(text).await?;
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    Ok(())
}

pub async fn speak_ollama(
    prompt: String,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let mut _index = 1;
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
