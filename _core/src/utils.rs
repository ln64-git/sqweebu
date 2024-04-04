use crate::io::process_response;
use crate::playback::PlaybackCommand;
use crate::AppState;
use _interface::{get_sentence_from_gpt, get_speech_from_api};
use base64::engine::general_purpose;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use surrealdb::Surreal;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub async fn check_empty_sink(
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<bool, Box<dyn Error>> {
    playback_send
        .send(PlaybackCommand::CheckSink)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    Ok(true)
}
pub async fn listen_audio_database(nexus: Arc<Mutex<AppState>>) -> Result<(), Box<dyn Error>> {
    let mut last_played_index: i32 = 0;

    loop {
        let nexus_locked = nexus.lock().await;
        let audio_db = &nexus_locked.audio_db;

        let audio_entries = audio_db.select::<Vec<AudioEntry>>("audio").await?;
        drop(nexus_locked); // Release the lock as soon as it's no longer needed.

        // Filter entries after dropping nexus_locked to avoid borrow issues.
        let new_audio_entries = audio_entries
            .into_iter()
            .filter(|entry| entry.index > last_played_index)
            .collect::<Vec<_>>();

        for entry in new_audio_entries {
            // Re-acquire lock for each entry to send PlaybackCommand
            let nexus_locked = nexus.lock().await;
            nexus_locked
                .playback_send
                .send(PlaybackCommand::Play(entry.clone()))
                .await?;
            drop(nexus_locked); // Optionally, release lock immediately after use.

            last_played_index = entry.index; // Update last_played_index without borrow conflict.

            // Wait for the audio length duration before considering the next entry
            tokio::time::sleep(Duration::from_secs_f32(entry.audio_length)).await;
        }

        // Implement a more sophisticated mechanism to break out of the loop if necessary.
        tokio::time::sleep(Duration::from_secs(1)).await; // Simple delay before next iteration.
    }
}

pub async fn read_from_sentence(start_index: i32, nexus: Arc<Mutex<AppState>>) {
    let nexus_locked = nexus.lock().await;
    let audio_db = &nexus_locked.audio_db;

    // Assuming audio entries are already sorted and unique by index.
    if let Ok(audio_entries) = audio_db.select::<Vec<AudioEntry>>("audio").await {
        for entry in audio_entries.iter().filter(|e| e.index >= start_index) {
            if entry.text_finished {
                break;
            }
            let _ = nexus_locked
                .playback_send
                .send(PlaybackCommand::Play(entry.clone()))
                .await
                .expect("Failed to send play command");
        }
    } else {
        eprintln!("Error querying audio entries from index: {}", start_index);
    }
}

pub async fn speak_text(
    text: &str,
    speech_service: &str,
    audio_db: Surreal<surrealdb::engine::local::Db>,
    entry_finished: bool,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text, speech_service).await?;
    let audio_length = get_audio_length(audio_data.clone()).await?; // Calculate audio length.
    let encoded_data = general_purpose::STANDARD.encode(audio_data);

    // Now pass audio_length to add_audio_entry_to_db.
    add_audio_entry_to_db(text, encoded_data, audio_db, entry_finished, audio_length)
        .await
        .map_err(|e| e as Box<dyn Error>)?;

    Ok(())
}

pub async fn speak_gpt(
    text: String,
    chat_db: Surreal<surrealdb::engine::local::Db>,
    audio_db: Surreal<surrealdb::engine::local::Db>,
    gpt_service: &str,
    speech_service: &str,
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
        speak_text(&sentence, speech_service, audio_db.clone(), false)
            .await
            .map_err(|e| {
                eprintln!("Error in speak_gpt: {}", e);
                e
            })?;
    }
    speak_text("", speech_service, audio_db.clone(), true)
        .await
        .map_err(|e| {
            eprintln!("Error in speak_gpt: {}", e);
            e
        })?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioEntry {
    pub index: i32,
    pub text_content: String,
    pub audio_data: String,
    pub audio_length: f32,
    pub playback_active: bool,
    pub playback_elapsed: f32,
    pub text_finished: bool,
}

use minimp3::{Decoder, Frame};
use std::io::Cursor;

async fn get_audio_length(audio_data: Vec<u8>) -> Result<f32, Box<dyn Error>> {
    let cursor = Cursor::new(audio_data);
    let mut decoder = Decoder::new(cursor);
    let mut total_duration = 0f32;

    // Iterate over each frame in the MP3 file
    while let Ok(Frame {
        sample_rate,
        channels,
        data,
        ..
    }) = decoder.next_frame()
    {
        // Calculate the duration of the current frame in seconds
        let frame_duration = (data.len() as f32) / (sample_rate as f32 * channels as f32 * 2f32);
        total_duration += frame_duration;
    }

    // Return the total duration in seconds
    Ok(total_duration)
}

async fn add_audio_entry_to_db(
    text: &str,
    encoded_data: String,
    audio_db: Surreal<surrealdb::engine::local::Db>,
    text_finished: bool,
    audio_length: f32, // Include audio length as a parameter
) -> Result<(), Box<dyn Error>> {
    let highest_index: i32 = get_highest_index(&audio_db).await?;
    let new_index = highest_index + 1;

    let entry = AudioEntry {
        index: new_index,
        text_content: text.to_string(),
        audio_data: encoded_data,
        audio_length,
        playback_active: false,
        playback_elapsed: 0.0,
        text_finished,
    };
    let _: Result<Vec<AudioEntry>, Box<dyn Error>> = audio_db
        .create("audio")
        .content(&entry)
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
