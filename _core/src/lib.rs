// Export items from playback.rs
// region: --- Region Title
pub mod playback;
use _adapter::{
    azure::get_azure_audio_response, google::get_google_audio_response, ollama::ollama_generate_api,
};
use _interface::PlaybackCommand;
pub use playback::*;
use std::error::Error;
use tokio::sync::mpsc;
// endregion: --- Region Title

pub async fn speak_text(
    text: &str,
    service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    if service == "azure" {
        let audio_data = get_azure_audio_response(text).await?;
        let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    } else if service == "google" {
        let audio_data = get_google_audio_response(text).await?;
        let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    } else {
    }
    Ok(())
}

pub async fn speak_ollama(
    prompt: String,
    service: &str,
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
        speak_text(&sentence, service, &playback_send).await?;
    }

    Ok(())
}
