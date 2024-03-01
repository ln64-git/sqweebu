// Export items from playback.rs

// region: --- Region Title
use _adapter::{
    azure::get_azure_audio_response, google::get_google_audio_response, ollama::ollama_generate_api,
};
use _interface::PlaybackCommand;
use std::error::Error;
use tokio::sync::mpsc;
// endregion: --- Region Title

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    if speech_service == "azure" {
        let audio_data = get_azure_audio_response(text).await?;
        let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    } else if speech_service == "google" {
        let audio_data = get_google_audio_response(text).await?;
        let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    }
    Ok(())
}

pub async fn speak_gpt(
    prompt: String,
    gpt_service: String,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let mut _index = 1;
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);
    if gpt_service == "ollama" {
        tokio::spawn(async move {
            match ollama_generate_api(prompt.clone(), sentence_send).await {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to generate sentences: {}", e),
            }
        });
    } else if gpt_service == "openai" {
        tokio::spawn(async move {
            match ollama_generate_api(prompt.clone(), sentence_send).await {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to generate sentences: {}", e),
            }
        });
    }

    while let Some(sentence) = sentence_recv.recv().await {
        speak_text(&sentence, speech_service, &playback_send).await?;
    }

    Ok(())
}
