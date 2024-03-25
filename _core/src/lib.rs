// region: --- Region Title
use surrealdb::Surreal;
pub mod playback;
pub mod utils;
use chrono::{DateTime, Utc};
use playback::PlaybackCommand;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use utils::speak_gpt;
// endregion: --- Region Title
// region: --- AppState

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub current_sentence: Arc<Mutex<String>>,
    pub chat_db: Surreal<surrealdb::engine::local::Db>,
    pub audio_db: Surreal<surrealdb::engine::local::Db>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            current_sentence: self.current_sentence.clone(),
            chat_db: self.chat_db.clone(),
            audio_db: self.audio_db.clone(),
        }
    }
}

// endregion: --- AppState

pub async fn process_input(
    text: &str,
    // playback_send: &mpsc::Sender<PlaybackCommand>,
    chat_db: Surreal<surrealdb::engine::local::Db>,
    audio_db: Surreal<surrealdb::engine::local::Db>,
) -> Result<(), Box<dyn Error>> {
    let _ = match text {
        // input if input.starts_with("speak text") => {
        //     input_text = input[10..].to_owned(); // Store the text without the "speak text" prefix
        //     speak_text(&input[10..], "azure", playback_send).await
        // }
        input if input.starts_with("speak gpt") => {
            let _ = add_chat_entry_to_db("user".to_owned(), &chat_db, input[9..].to_owned()).await;
            speak_gpt(
                input[9..].to_owned(),
                chat_db.clone(),
                audio_db.clone(),
                "ollama",
                "azure",
            )
            .await
        }
        _ => Ok(()),
    };
    Ok(())
}

pub async fn process_response(
    sentence: String,
    db: Surreal<surrealdb::engine::local::Db>,
) -> Result<(), Box<dyn Error>> {
    let _ = add_chat_entry_to_db("gpt".to_owned(), &db, sentence).await;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatEntry {
    source: String,
    timestamp: DateTime<Utc>,
    content: String,
}

async fn add_chat_entry_to_db(
    source: String,
    chat_db: &Surreal<surrealdb::engine::local::Db>,
    content: String,
) -> Result<(), Box<dyn Error>> {
    let content = ChatEntry {
        source,
        timestamp: Utc::now(),
        content,
    };
    let _: Option<Vec<ChatEntry>> = match chat_db.create("chat").content(content).await {
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
