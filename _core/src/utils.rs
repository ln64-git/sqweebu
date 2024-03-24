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
    let mut last_played_index: i32 = 0; // Initialize with 0 or load this from a persisted source

    loop {
        let app_state_locked = app_state.lock().await;
        let audio_db = &app_state_locked.audio_db;

        match audio_db.select::<Vec<AudioEntry>>("audio").await {
            Ok(mut audio_entries) => {
                // Sort the audio_entries by their index in ascending order
                audio_entries.sort_by_key(|entry| entry.index);

                // Filter out entries that have already been played by checking against last_played_index
                let new_audio_entries = audio_entries
                    .into_iter()
                    .filter(|entry| entry.index > last_played_index)
                    .collect::<Vec<AudioEntry>>();

                for entry in new_audio_entries {
                    let audio_data = BASE64_STANDARD
                        .decode(entry.audio_data.as_bytes())
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

                    app_state_locked
                        .playback_send
                        .send(PlaybackCommand::Play(audio_data))
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

                    println!("Played audio entry with index: {}", entry.index);
                    // Update last_played_index with the index of the last played entry
                    last_played_index = entry.index;
                }
            }
            Err(e) => {
                eprintln!("Error querying audio entries: {}", e);
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }

        drop(app_state_locked);
        // sleep(Duration::from_secs(5)).await; // Maintain this sleep to prevent constant querying
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
    index: i32,
    text_content: String,
    audio_data: String,
}

async fn add_audio_entry_to_db(
    text: &str,
    encoded_data: String,
    audio_db: Surreal<surrealdb::engine::local::Db>,
) -> Result<(), Box<dyn Error>> {
    let highest_index: i32 = get_highest_index(&audio_db).await?;
    let new_index = highest_index + 1;
    let audio = AudioEntry {
        index: new_index,
        text_content: text.to_string(),
        audio_data: encoded_data,
    };
    let _: Result<Vec<AudioEntry>, Box<dyn Error>> = audio_db
        .create("audio")
        .content(&audio)
        .await
        .map_err(|e| e.into());

    Ok(())
}

async fn get_highest_index(
    audio_db: &Surreal<surrealdb::engine::local::Db>,
) -> Result<i32, Box<dyn Error>> {
    let mut highest_index = 0;
    match audio_db.select::<Vec<AudioEntry>>("audio").await {
        Ok(audio_entries) => {
            for entry in audio_entries {
                if entry.index > highest_index {
                    highest_index = entry.index
                }
            }
        }
        Err(e) => {
            eprintln!("Error querying audio entries: {}", e);
        }
    }
    Ok(highest_index)
}
