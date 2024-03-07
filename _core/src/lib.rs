// region: --- Region Title
pub mod playback;
use _interface::{get_sentence_from_api, get_speech_from_api};
use playback::PlaybackCommand;
use std::error::Error;
use tokio::sync::mpsc;
// endregion: --- Region Title

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
        }
    }
}

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    Ok(())
}

pub async fn speak_gpt(
    prompt: String,
    gpt_service: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);
    let gpt_service_cloned = gpt_service.to_string();
    tokio::spawn(async move {
        match get_sentence_from_api(prompt.clone(), &gpt_service_cloned, sentence_send).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to generate sentences: {}", e),
        }
    });

    while let Some(sentence) = sentence_recv.recv().await {
        speak_text(&sentence, speech_service, &playback_send).await?;
    }
    Ok(())
}
