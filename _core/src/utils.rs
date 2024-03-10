use crate::process_response;
use surrealdb::Surreal;
use _interface::{ get_sentence_from_gpt, get_speech_from_api };
use playback::PlaybackCommand;
use std::error::Error;
use tokio::sync::mpsc;

use crate::playback;

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    Ok(())
}

pub async fn speak_gpt(
    text: String,
    gpt_service: &str,
    db: Surreal<surrealdb::engine::local::Db>
    // speech_service: &str,
    // playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let (sentence_send, mut sentence_recv) = mpsc::channel::<String>(32);
    let gpt_service_cloned = gpt_service.to_string();
    tokio::spawn(async move {
        match get_sentence_from_gpt(text.clone(), &gpt_service_cloned, sentence_send).await {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to generate sentences: {}", e),
        }
    });

    while let Some(sentence) = sentence_recv.recv().await {
        let _ = process_response(sentence, db.clone()).await;
        // speak_text(&sentence, speech_service, &playback_send).await?;
    }
    Ok(())
}
