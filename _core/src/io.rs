use crate::{utils::speak_gpt, Surreal};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

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
    pub index: i32,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

async fn add_chat_entry_to_db(
    source: String,
    chat_db: &Surreal<surrealdb::engine::local::Db>,
    content: String,
) -> Result<(), Box<dyn Error>> {
    let latest_index = get_latest_index(chat_db).await?;
    let new_index = latest_index + 1;

    let entry = ChatEntry {
        index: new_index,
        source,
        timestamp: Utc::now(),
        content,
    };

    let _: Option<Vec<ChatEntry>> = match chat_db.create("chat").content(entry).await {
        Ok(records) => {
            records.clone().into_iter().next();
            Some(records)
        }
        Err(e) => {
            println!("Error adding chat entry to DB: {:?}", e);
            None
        }
    };
    Ok(())
}

async fn get_latest_index(
    chat_db: &Surreal<surrealdb::engine::local::Db>,
) -> Result<i32, Box<dyn Error>> {
    let chat_entries_result = chat_db.select::<Vec<ChatEntry>>("chat").await;
    match chat_entries_result {
        Ok(chat_entries) => {
            if let Some(max_entry) = chat_entries.iter().max_by_key(|entry| entry.index) {
                Ok(max_entry.index)
            } else {
                Ok(0) // Return 0 if no entries are found
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}
