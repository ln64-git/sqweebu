// region: --- Region Title
use surrealdb::Surreal;
pub mod playback;
pub mod utils;
use chrono::{DateTime, Utc};
use playback::PlaybackCommand;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::mpsc;
use utils::speak_gpt;
// endregion: --- Region Title
// region: --- AppState

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
    pub db: Surreal<surrealdb::engine::local::Db>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
            db: self.db.clone(), // Clone the database connection as well
        }
    }
}

// endregion: --- AppState

pub async fn process_input(
    text: &str,
    db: Surreal<surrealdb::engine::local::Db>,
) -> Result<(), Box<dyn Error>> {
    let mut input_text = text.to_owned();

    let _ = match text {
        // input if input.starts_with("speak text") => {
        //     input_text = input[10..].to_owned(); // Store the text without the "speak text" prefix
        //     speak_text(&input[10..], "azure", playback_send).await
        // }
        input if input.starts_with("speak gpt") => {
            input_text = input[9..].to_owned(); // Store the text without the "speak gpt" prefix
            speak_gpt((&input[9..]).to_owned(), "ollama", db.clone()).await
        }
        _ => Ok(()),
    };

    let _ = add_chat_entry_to_db("user".to_owned(), &db, input_text).await;
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
    db: &Surreal<surrealdb::engine::local::Db>,
    content: String,
) -> Result<(), Box<dyn Error>> {
    let content = ChatEntry {
        source,
        timestamp: Utc::now(),
        content,
    };
    let _ = db.use_ns("user3").use_db("user3").await?;
    let _: Option<Vec<ChatEntry>> = match db.create("chat").content(content).await {
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
