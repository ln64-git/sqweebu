use crate::playback::PlaybackCommand;
use crate::process_response;
use crate::AppState;
use _interface::{get_sentence_from_gpt, get_speech_from_api};
use base64::engine::general_purpose;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::time::Duration;
use surrealdb::Surreal;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    audio_db: Surreal<surrealdb::engine::local::Db>,
    // playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;

    // Using the STANDARD engine for base64 encoding
    let encoded_data = general_purpose::STANDARD.encode(&audio_data);

    add_audio_entry_to_db(text, encoded_data, audio_db)
        .await
        .map_err(|e| e as Box<dyn Error>)?;

    // let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;

    Ok(())
}

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn listen_audio_database(app_state: Arc<Mutex<AppState>>) -> Result<(), Box<dyn Error>> {
    loop {
        let app_state_locked = app_state.lock().await;
        let audio_db = &app_state_locked.audio_db;

        match audio_db.select::<Vec<AudioEntry>>("audio").await {
            Ok(audio_entries) => {
                for entry in audio_entries {
                    let audio_data = BASE64_STANDARD
                        .decode(entry.audio_data.as_bytes())
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

                    app_state_locked
                        .playback_send
                        .send(PlaybackCommand::Play(audio_data))
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                }
            }
            Err(e) => {
                eprintln!("Error querying audio entries: {}", e);
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }

        drop(app_state_locked);
        sleep(Duration::from_secs(5)).await;
    }
}

pub async fn speak_gpt(
    text: String,
    chat_db: Surreal<surrealdb::engine::local::Db>,
    audio_db: Surreal<surrealdb::engine::local::Db>,
    gpt_service: &str,
    speech_service: &str,
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
        let _ = process_response(sentence.clone(), chat_db.clone()).await;
        speak_text(&sentence, speech_service, audio_db.clone()).await?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioEntry {
    text_content: String,
    audio_data: String, // Base64 encoded audio data
}

async fn add_audio_entry_to_db(
    text: &str,
    encoded_data: String,
    audio_db: Surreal<surrealdb::engine::local::Db>,
) -> Result<(), Box<dyn Error>> {
    // Construct the audio record with the text content and encoded audio data
    let audio = AudioEntry {
        text_content: text.to_string(),
        audio_data: encoded_data,
    };

    // Insert the encoded audio data into the database
    let _: Result<Vec<AudioEntry>, Box<dyn Error>> = audio_db
        .create("audio")
        .content(&audio)
        .await
        .map_err(|e| e.into());

    let _: Option<Vec<AudioEntry>> = match audio_db.create("audio").content(&audio).await {
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
