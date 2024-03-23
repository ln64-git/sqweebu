use crate::playback;
use crate::process_response;
use _interface::{get_sentence_from_gpt, get_speech_from_api};
use base64::engine::general_purpose;
use base64::Engine;
use playback::PlaybackCommand;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use tokio::sync::mpsc;

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;
    let audio_db = Surreal::new::<Mem>(()).await?;
    let _ = audio_db.use_ns("user3").use_db("audio").await;

    // Using the STANDARD engine for base64 encoding
    let encoded_data = general_purpose::STANDARD.encode(&audio_data);
    
    let _ = add_audio_entry_to_db(text, encoded_data);
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;

    Ok(())
}

pub async fn speak_gpt(
    text: String,
    db: Surreal<surrealdb::engine::local::Db>,
    gpt_service: &str,
    speech_service: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
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
        let _ = process_response(sentence.clone(), db.clone()).await;
        speak_text(&sentence, speech_service, playback_send).await?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AudioEntry {
    text_content: String,
    audio_data: String, // Base64 encoded audio data
}

async fn add_audio_entry_to_db(text: &str, encoded_data: String) -> Result<(), Box<dyn Error>> {
    let audio_db = Surreal::new::<Mem>(()).await?;
    let _ = audio_db.use_ns("user3").use_db("audio").await;

    // Construct the audio record with the text content and encoded audio data
    let audio = AudioEntry {
        text_content: text.to_string(),
        audio_data: encoded_data,
    };

    // Insert the encoded audio data into the database
    let _: Result<Vec<AudioEntry>, Box<dyn Error>> = audio_db
        .create("Audio")
        .content(&audio)
        .await
        .map_err(|e| e.into());

    let _: Option<Vec<AudioEntry>> = match audio_db.create("Audio").content(&audio).await {
        Ok(records) => {
            records.clone().into_iter().next();
            Some(records)
        }
        Err(e) => {
            println!("PROCESS_INPUT - Error: {:?}", e);
            None
        }
    };
    Ok(())
}
